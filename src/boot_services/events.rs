// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Event and Timer Services

use crate::boot_services::BootServices;
use crate::ffi::*;

// Event types
pub const EVT_TIMER: u32 = 0x80000000;
pub const EVT_RUNTIME: u32 = 0x40000000;
pub const EVT_NOTIFY_WAIT: u32 = 0x00000100;
pub const EVT_NOTIFY_SIGNAL: u32 = 0x00000200;
pub const EVT_SIGNAL_EXIT_BOOT_SERVICES: u32 = 0x00000201;
pub const EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE: u32 = 0x60000202;

// Timer delays
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimerDelay {
    TimerCancel = 0,
    TimerPeriodic = 1,
    TimerRelative = 2,
}

/// Event notification callback type
pub type EventNotifyFn = unsafe extern "efiapi" fn(event: Event, context: *mut core::ffi::c_void);

/// Event wrapper for safe event management
pub struct EventWrapper<'a> {
    bs: &'a BootServices,
    event: Event,
}

impl<'a> EventWrapper<'a> {
    /// Create a new event
    pub unsafe fn create(
        bs: &'a BootServices,
        event_type: u32,
        notify_tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: *mut core::ffi::c_void,
    ) -> Result<Self, Status> {
        let mut event = core::ptr::null_mut();
        let notify_fn =
            notify_function.map_or(core::ptr::null_mut(), |f| f as *mut core::ffi::c_void);

        let status = (bs.create_event)(
            event_type,
            notify_tpl,
            notify_fn,
            notify_context,
            &mut event,
        );

        if status == EFI_SUCCESS {
            Ok(EventWrapper { bs, event })
        } else {
            Err(status)
        }
    }

    /// Create a new event (ex version with event group)
    pub unsafe fn create_ex(
        bs: &'a BootServices,
        event_type: u32,
        notify_tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: *mut core::ffi::c_void,
        event_group: Option<&Guid>,
    ) -> Result<Self, Status> {
        let mut event = core::ptr::null_mut();
        let notify_fn =
            notify_function.map_or(core::ptr::null_mut(), |f| f as *mut core::ffi::c_void);
        let group_ptr = event_group.map_or(core::ptr::null(), |g| g as *const _);

        let status = (bs.create_event_ex)(
            event_type,
            notify_tpl,
            notify_fn,
            notify_context,
            group_ptr,
            &mut event,
        );

        if status == EFI_SUCCESS {
            Ok(EventWrapper { bs, event })
        } else {
            Err(status)
        }
    }

    /// Set timer
    pub unsafe fn set_timer(&self, timer_type: TimerDelay, trigger_time: u64) -> Status {
        (self.bs.set_timer)(self.event, timer_type as u32, trigger_time)
    }

    /// Signal the event
    pub unsafe fn signal(&self) -> Status {
        (self.bs.signal_event)(self.event)
    }

    /// Check the event status
    pub unsafe fn check(&self) -> Status {
        (self.bs.check_event)(self.event)
    }

    /// Wait for event
    pub unsafe fn wait(&self) -> Result<usize, Status> {
        let mut index = 0;
        let status = (self.bs.wait_for_event)(1, &self.event as *const _ as *mut Event, &mut index);

        if status == EFI_SUCCESS {
            Ok(index)
        } else {
            Err(status)
        }
    }

    /// Get the raw event handle
    pub fn as_raw(&self) -> Event {
        self.event
    }
}

impl Drop for EventWrapper<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = (self.bs.close_event)(self.event);
        }
    }
}

/// Timer wrapper for easy timer management
pub struct Timer<'a> {
    event: EventWrapper<'a>,
}

impl<'a> Timer<'a> {
    /// Create a new timer
    pub unsafe fn create(bs: &'a BootServices, tpl: Tpl) -> Result<Self, Status> {
        let event = EventWrapper::create(bs, EVT_TIMER, tpl, None, core::ptr::null_mut())?;

        Ok(Timer { event })
    }

    /// Set timer to fire after delay (in 100ns units)
    pub unsafe fn set_relative(&self, delay_100ns: u64) -> Status {
        self.event.set_timer(TimerDelay::TimerRelative, delay_100ns)
    }

    /// Set timer to fire periodically (in 100ns units)
    pub unsafe fn set_periodic(&self, period_100ns: u64) -> Status {
        self.event
            .set_timer(TimerDelay::TimerPeriodic, period_100ns)
    }

    /// Cancel the timer
    pub unsafe fn cancel(&self) -> Status {
        self.event.set_timer(TimerDelay::TimerCancel, 0)
    }

    /// Wait for timer to fire
    pub unsafe fn wait(&self) -> Result<(), Status> {
        self.event.wait().map(|_| ())
    }

    /// Check if timer has fired
    pub unsafe fn check(&self) -> bool {
        self.event.check() == EFI_SUCCESS
    }
}

/// Helper functions for time conversions
pub mod time_utils {
    /// Convert milliseconds to 100ns units
    pub const fn ms_to_100ns(ms: u64) -> u64 {
        ms * 10_000
    }

    /// Convert microseconds to 100ns units
    pub const fn us_to_100ns(us: u64) -> u64 {
        us * 10
    }

    /// Convert seconds to 100ns units
    pub const fn sec_to_100ns(sec: u64) -> u64 {
        sec * 10_000_000
    }

    /// Convert 100ns units to milliseconds
    pub const fn ns100_to_ms(ns100: u64) -> u64 {
        ns100 / 10_000
    }

    /// Convert 100ns units to microseconds
    pub const fn ns100_to_us(ns100: u64) -> u64 {
        ns100 / 10
    }

    /// Convert 100ns units to seconds
    pub const fn ns100_to_sec(ns100: u64) -> u64 {
        ns100 / 10_000_000
    }
}
