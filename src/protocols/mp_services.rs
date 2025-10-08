// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Multi-Processor (MP) Services Protocol

use crate::ffi::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;

/// EFI_MP_SERVICES_PROTOCOL_GUID
pub const MP_SERVICES_PROTOCOL_GUID: Guid = Guid::new(
    0x3fdda605,
    0xa76e,
    0x4f46,
    [0xad, 0x29, 0x12, 0xf4, 0x53, 0x1b, 0x3d, 0x08],
);

/// Processor ID
pub type ProcessorId = Uintn;

/// Extended Topology
#[repr(C)]
pub struct ExtendedProcessorInformation {
    pub location: ProcessorLocation,
}

/// Processor Location
#[repr(C)]
pub struct ProcessorLocation {
    pub package: Uint32,
    pub core: Uint32,
    pub thread: Uint32,
}

/// Processor Information
#[repr(C)]
pub struct ProcessorInformation {
    pub processor_id: Uint64,
    pub status_flag: Uint32,
    pub location: ProcessorLocation,
    pub extended_information: ExtendedProcessorInformation,
}

/// Processor Status Flags
pub const PROCESSOR_AS_BSP_BIT: Uint32 = 0x00000001;
pub const PROCESSOR_ENABLED_BIT: Uint32 = 0x00000002;
pub const PROCESSOR_HEALTH_STATUS_BIT: Uint32 = 0x00000004;

/// Processor Procedure
pub type ProcessorProcedure = unsafe extern "efiapi" fn(buffer: *mut core::ffi::c_void);

/// End of Procedure Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EndOfProcedureType {
    Blocking = 0,
    NonBlocking = 1,
}

/// EFI_MP_SERVICES_PROTOCOL
#[repr(C)]
pub struct MpServicesProtocol {
    pub get_number_of_processors: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        number_of_processors: *mut Uintn,
        number_of_enabled_processors: *mut Uintn,
    ) -> Status,
    pub get_processor_info: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        processor_number: Uintn,
        processor_info_buffer: *mut ProcessorInformation,
    ) -> Status,
    pub startup_all_aps: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        procedure: ProcessorProcedure,
        single_thread: Boolean,
        wait_event: Event,
        timeout_in_microseconds: Uintn,
        procedure_argument: *mut core::ffi::c_void,
        failed_cpu_list: *mut *mut Uintn,
    ) -> Status,
    pub startup_this_ap: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        procedure: ProcessorProcedure,
        processor_number: Uintn,
        wait_event: Event,
        timeout_in_microseconds: Uintn,
        procedure_argument: *mut core::ffi::c_void,
        finished: *mut Boolean,
    ) -> Status,
    pub switch_bsp: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        processor_number: Uintn,
        enable_old_bsp: Boolean,
    ) -> Status,
    pub enable_disable_ap: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        processor_number: Uintn,
        enable_ap: Boolean,
        health_flag: *mut Uint32,
    ) -> Status,
    pub who_am_i: unsafe extern "efiapi" fn(
        this: *mut MpServicesProtocol,
        processor_number: *mut Uintn,
    ) -> Status,
}

impl MpServicesProtocol {
    /// Get the number of processors and enabled processors
    pub unsafe fn get_number_of_processors(&mut self) -> Result<(usize, usize), Status> {
        let mut total = 0;
        let mut enabled = 0;

        let status = (self.get_number_of_processors)(self, &mut total, &mut enabled);

        if status == EFI_SUCCESS {
            Ok((total, enabled))
        } else {
            Err(status)
        }
    }

    /// Get information about a specific processor
    pub unsafe fn get_processor_info(
        &mut self,
        processor_number: usize,
    ) -> Result<ProcessorInformation, Status> {
        let mut info: ProcessorInformation = core::mem::zeroed();
        let status = (self.get_processor_info)(self, processor_number, &mut info);

        if status == EFI_SUCCESS {
            Ok(info)
        } else {
            Err(status)
        }
    }

    /// Execute a procedure on all Application Processors
    pub unsafe fn startup_all_aps(
        &mut self,
        procedure: ProcessorProcedure,
        single_thread: bool,
        timeout_us: usize,
        argument: *mut core::ffi::c_void,
    ) -> Result<(), Status> {
        let status = (self.startup_all_aps)(
            self,
            procedure,
            single_thread as Boolean,
            core::ptr::null_mut(),
            timeout_us,
            argument,
            core::ptr::null_mut(),
        );

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Execute a procedure on a specific Application Processor
    pub unsafe fn startup_this_ap(
        &mut self,
        procedure: ProcessorProcedure,
        processor_number: usize,
        timeout_us: usize,
        argument: *mut core::ffi::c_void,
    ) -> Result<(), Status> {
        let status = (self.startup_this_ap)(
            self,
            procedure,
            processor_number,
            core::ptr::null_mut(),
            timeout_us,
            argument,
            core::ptr::null_mut(),
        );

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Switch the Bootstrap Processor
    pub unsafe fn switch_bsp(&mut self, new_bsp: usize, enable_old_bsp: bool) -> Status {
        (self.switch_bsp)(self, new_bsp, enable_old_bsp as Boolean)
    }

    /// Enable or disable an Application Processor
    pub unsafe fn enable_disable_ap(
        &mut self,
        processor_number: usize,
        enable: bool,
    ) -> Result<u32, Status> {
        let mut health_flag = 0;
        let status =
            (self.enable_disable_ap)(self, processor_number, enable as Boolean, &mut health_flag);

        if status == EFI_SUCCESS {
            Ok(health_flag)
        } else {
            Err(status)
        }
    }

    /// Get the current processor number
    pub unsafe fn who_am_i(&mut self) -> Result<usize, Status> {
        let mut processor_number = 0;
        let status = (self.who_am_i)(self, &mut processor_number);

        if status == EFI_SUCCESS {
            Ok(processor_number)
        } else {
            Err(status)
        }
    }

    /// Check if a processor is the BSP
    pub fn is_bsp(info: &ProcessorInformation) -> bool {
        (info.status_flag & PROCESSOR_AS_BSP_BIT) != 0
    }

    /// Check if a processor is enabled
    pub fn is_enabled(info: &ProcessorInformation) -> bool {
        (info.status_flag & PROCESSOR_ENABLED_BIT) != 0
    }

    /// Check if a processor is healthy
    pub fn is_healthy(info: &ProcessorInformation) -> bool {
        (info.status_flag & PROCESSOR_HEALTH_STATUS_BIT) != 0
    }
}

/// MP Services Utilities
pub mod mp_utils {
    use super::*;

    /// Get all processor information
    pub unsafe fn get_all_processors(
        mp: &mut MpServicesProtocol,
    ) -> Result<Vec<ProcessorInformation>, Status> {
        let (total, _) = mp.get_number_of_processors()?;
        let mut processors = Vec::with_capacity(total);

        for i in 0..total {
            match mp.get_processor_info(i) {
                Ok(info) => processors.push(info),
                Err(_) => continue,
            }
        }

        Ok(processors)
    }

    /// Find the BSP processor number
    pub unsafe fn find_bsp(mp: &mut MpServicesProtocol) -> Result<usize, Status> {
        let processors = get_all_processors(mp)?;

        for (i, info) in processors.iter().enumerate() {
            if MpServicesProtocol::is_bsp(info) {
                return Ok(i);
            }
        }

        Err(EFI_NOT_FOUND)
    }

    /// Get all enabled processor numbers
    pub unsafe fn get_enabled_processors(
        mp: &mut MpServicesProtocol,
    ) -> Result<Vec<usize>, Status> {
        let processors = get_all_processors(mp)?;
        let mut enabled = Vec::new();

        for (i, info) in processors.iter().enumerate() {
            if MpServicesProtocol::is_enabled(info) {
                enabled.push(i);
            }
        }

        Ok(enabled)
    }

    /// Execute procedure on all APs and wait for completion
    pub unsafe fn execute_on_all_aps_sync(
        mp: &mut MpServicesProtocol,
        procedure: ProcessorProcedure,
        argument: *mut core::ffi::c_void,
        timeout_us: usize,
    ) -> Result<(), Status> {
        mp.startup_all_aps(procedure, false, timeout_us, argument)
    }

    /// Execute procedure on specific AP and wait for completion
    pub unsafe fn execute_on_ap_sync(
        mp: &mut MpServicesProtocol,
        processor_number: usize,
        procedure: ProcessorProcedure,
        argument: *mut core::ffi::c_void,
        timeout_us: usize,
    ) -> Result<(), Status> {
        mp.startup_this_ap(procedure, processor_number, timeout_us, argument)
    }
}

/// Example processor procedure
#[no_mangle]
pub unsafe extern "efiapi" fn example_processor_procedure(buffer: *mut core::ffi::c_void) {
    // This would be implemented by the user
    // Example: increment a counter, perform calculations, etc.
    if !buffer.is_null() {
        let counter = buffer as *mut usize;
        *counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_flags() {
        assert_eq!(PROCESSOR_AS_BSP_BIT, 0x01);
        assert_eq!(PROCESSOR_ENABLED_BIT, 0x02);
        assert_eq!(PROCESSOR_HEALTH_STATUS_BIT, 0x04);
    }

    #[test]
    fn test_processor_info_size() {
        assert!(core::mem::size_of::<ProcessorInformation>() > 0);
    }

    #[test]
    fn test_flag_checking() {
        let info = ProcessorInformation {
            processor_id: 0,
            status_flag: PROCESSOR_AS_BSP_BIT | PROCESSOR_ENABLED_BIT,
            location: ProcessorLocation {
                package: 0,
                core: 0,
                thread: 0,
            },
            extended_information: ExtendedProcessorInformation {
                location: ProcessorLocation {
                    package: 0,
                    core: 0,
                    thread: 0,
                },
            },
        };

        assert!(MpServicesProtocol::is_bsp(&info));
        assert!(MpServicesProtocol::is_enabled(&info));
        assert!(!MpServicesProtocol::is_healthy(&info));
    }
}
