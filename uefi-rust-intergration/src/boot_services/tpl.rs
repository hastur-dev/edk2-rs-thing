// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Task Priority Level (TPL) Management Utilities

use crate::ffi::*;

/// Task Priority Levels
pub const TPL_APPLICATION: Tpl = 4;
pub const TPL_CALLBACK: Tpl = 8;
pub const TPL_NOTIFY: Tpl = 16;
pub const TPL_HIGH_LEVEL: Tpl = 31;

/// TPL Guard - RAII wrapper for TPL management
///
/// This structure provides safe TPL (Task Priority Level) management with automatic
/// restoration when the guard goes out of scope. This prevents TPL leaks and ensures
/// proper nesting.
///
/// # Example
/// ```no_run
/// use uefi_rust::boot_services::tpl::TplGuard;
///
/// // Raise TPL to CALLBACK level
/// let guard = unsafe { TplGuard::raise(TPL_CALLBACK) };
/// // Critical section - interrupts at lower priority are blocked
/// // ...
/// // TPL automatically restored when guard drops
/// ```
pub struct TplGuard {
    old_tpl: Tpl,
}

impl TplGuard {
    /// Raise TPL to the specified level and return a guard
    ///
    /// # Safety
    /// This function is unsafe because:
    /// - Must have valid Boot Services table pointer
    /// - Caller must ensure proper TPL ordering
    /// - Must not be called at TPL_HIGH_LEVEL
    ///
    /// # Arguments
    /// * `new_tpl` - The new TPL level to raise to
    ///
    /// # Returns
    /// A TplGuard that will restore the old TPL when dropped
    pub unsafe fn raise(new_tpl: Tpl) -> Self {
        let boot_services = get_boot_services();
        let old_tpl = ((*boot_services).raise_tpl)(new_tpl);
        TplGuard { old_tpl }
    }

    /// Get the old TPL level that will be restored
    pub fn old_tpl(&self) -> Tpl {
        self.old_tpl
    }

    /// Manually restore the old TPL and consume the guard
    ///
    /// # Safety
    /// This function is unsafe because it requires valid Boot Services
    pub unsafe fn restore(self) {
        // Drop will handle restoration
        drop(self);
    }
}

impl Drop for TplGuard {
    fn drop(&mut self) {
        unsafe {
            let boot_services = get_boot_services();
            ((*boot_services).restore_tpl)(self.old_tpl);
        }
    }
}

/// TPL Utilities
pub mod tpl_utils {
    use super::*;

    /// Get current TPL by raising to HIGH_LEVEL and immediately restoring
    ///
    /// # Safety
    /// Requires valid Boot Services table
    pub unsafe fn get_current_tpl() -> Tpl {
        let boot_services = get_boot_services();
        let current = ((*boot_services).raise_tpl)(TPL_HIGH_LEVEL);
        ((*boot_services).restore_tpl)(current);
        current
    }

    /// Check if current TPL is at or above the specified level
    ///
    /// # Safety
    /// Requires valid Boot Services table
    pub unsafe fn is_tpl_at_least(level: Tpl) -> bool {
        get_current_tpl() >= level
    }

    /// Execute a function at a specific TPL level
    ///
    /// This is a convenience function that handles TPL management automatically.
    ///
    /// # Safety
    /// - Requires valid Boot Services table
    /// - Function must be safe to execute at the specified TPL
    ///
    /// # Example
    /// ```no_run
    /// let result = unsafe {
    ///     execute_at_tpl(TPL_NOTIFY, || {
    ///         // Code executed at TPL_NOTIFY
    ///         42
    ///     })
    /// };
    /// ```
    pub unsafe fn execute_at_tpl<F, R>(tpl: Tpl, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = TplGuard::raise(tpl);
        f()
    }

    /// Check if we're at APPLICATION level
    pub unsafe fn is_at_application_level() -> bool {
        get_current_tpl() == TPL_APPLICATION
    }

    /// Check if we're at CALLBACK level or higher
    pub unsafe fn is_at_callback_level() -> bool {
        get_current_tpl() >= TPL_CALLBACK
    }

    /// Check if we're at NOTIFY level or higher
    pub unsafe fn is_at_notify_level() -> bool {
        get_current_tpl() >= TPL_NOTIFY
    }

    /// Check if we're at HIGH_LEVEL
    pub unsafe fn is_at_high_level() -> bool {
        get_current_tpl() == TPL_HIGH_LEVEL
    }
}

/// Critical Section Guard - Execute code at TPL_NOTIFY
///
/// This is a specialized guard for critical sections that need to block
/// most interrupts and callbacks.
pub struct CriticalSection {
    _guard: TplGuard,
}

impl CriticalSection {
    /// Enter a critical section by raising TPL to NOTIFY
    ///
    /// # Safety
    /// Requires valid Boot Services table
    pub unsafe fn enter() -> Self {
        CriticalSection {
            _guard: TplGuard::raise(TPL_NOTIFY),
        }
    }
}

/// Callback Guard - Execute code at TPL_CALLBACK
///
/// This is a specialized guard for code that needs to run at callback priority
/// but still allow notify-level events.
pub struct CallbackGuard {
    _guard: TplGuard,
}

impl CallbackGuard {
    /// Enter callback priority level
    ///
    /// # Safety
    /// Requires valid Boot Services table
    pub unsafe fn enter() -> Self {
        CallbackGuard {
            _guard: TplGuard::raise(TPL_CALLBACK),
        }
    }
}

/// Get the global Boot Services pointer
///
/// # Safety
/// This assumes Boot Services have been properly initialized
unsafe fn get_boot_services() -> *mut BootServices {
    // In a real implementation, this would get the boot services from
    // the system table stored globally during initialization.
    // For now, we'll use a placeholder that should be replaced with
    // actual global state management.
    extern "C" {
        static mut BOOT_SERVICES: *mut BootServices;
    }
    BOOT_SERVICES
}

/// Scoped TPL elevation macro
///
/// This macro provides a convenient way to execute code at an elevated TPL
/// with automatic restoration.
///
/// # Example
/// ```no_run
/// with_tpl!(TPL_CALLBACK, {
///     // Code executed at TPL_CALLBACK
///     do_critical_work();
/// });
/// // TPL automatically restored here
/// ```
#[macro_export]
macro_rules! with_tpl {
    ($tpl:expr, $code:block) => {{
        let _guard = unsafe { $crate::boot_services::tpl::TplGuard::raise($tpl) };
        $code
    }};
}

/// Critical section macro
///
/// Execute code in a critical section (TPL_NOTIFY)
///
/// # Example
/// ```no_run
/// critical_section!({
///     // Code executed at TPL_NOTIFY
///     modify_shared_state();
/// });
/// ```
#[macro_export]
macro_rules! critical_section {
    ($code:block) => {{
        let _cs = unsafe { $crate::boot_services::tpl::CriticalSection::enter() };
        $code
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tpl_constants() {
        assert_eq!(TPL_APPLICATION, 4);
        assert_eq!(TPL_CALLBACK, 8);
        assert_eq!(TPL_NOTIFY, 16);
        assert_eq!(TPL_HIGH_LEVEL, 31);
    }

    #[test]
    fn test_tpl_ordering() {
        assert!(TPL_APPLICATION < TPL_CALLBACK);
        assert!(TPL_CALLBACK < TPL_NOTIFY);
        assert!(TPL_NOTIFY < TPL_HIGH_LEVEL);
    }
}
