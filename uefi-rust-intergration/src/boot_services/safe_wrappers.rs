// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Safe wrappers around Boot Services

use crate::ffi::*;
use crate::boot_services::BootServices;
use core::ptr::null_mut;

/// Result type for UEFI operations
pub type Result<T> = core::result::Result<T, Status>;

/// Safe wrapper for Boot Services
pub struct BootServicesWrapper<'a> {
    bs: &'a BootServices,
}

impl<'a> BootServicesWrapper<'a> {
    /// Create a new wrapper from a reference to Boot Services
    pub fn new(bs: &'a BootServices) -> Self {
        Self { bs }
    }

    /// Allocate memory pages
    pub fn allocate_pages(
        &self,
        alloc_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
    ) -> Result<PhysicalAddress> {
        let mut addr: PhysicalAddress = 0;
        let status = unsafe {
            (self.bs.allocate_pages)(alloc_type, memory_type, pages, &mut addr)
        };

        if status == EFI_SUCCESS {
            Ok(addr)
        } else {
            Err(status)
        }
    }

    /// Free memory pages
    pub fn free_pages(&self, memory: PhysicalAddress, pages: usize) -> Result<()> {
        let status = unsafe { (self.bs.free_pages)(memory, pages) };

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Allocate memory pool
    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut u8> {
        let mut buffer: *mut core::ffi::c_void = null_mut();
        let status = unsafe {
            (self.bs.allocate_pool)(pool_type, size, &mut buffer)
        };

        if status == EFI_SUCCESS {
            Ok(buffer as *mut u8)
        } else {
            Err(status)
        }
    }

    /// Free memory pool
    pub fn free_pool(&self, buffer: *mut u8) -> Result<()> {
        let status = unsafe {
            (self.bs.free_pool)(buffer as *mut core::ffi::c_void)
        };

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Stall execution for a number of microseconds
    pub fn stall(&self, microseconds: usize) -> Result<()> {
        let status = unsafe { (self.bs.stall)(microseconds) };

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Locate a protocol interface
    pub fn locate_protocol(&self, protocol: &Guid) -> Result<*mut core::ffi::c_void> {
        let mut interface: *mut core::ffi::c_void = null_mut();
        let status = unsafe {
            (self.bs.locate_protocol)(protocol, null_mut(), &mut interface)
        };

        if status == EFI_SUCCESS {
            Ok(interface)
        } else {
            Err(status)
        }
    }

    /// Exit boot services
    pub fn exit_boot_services(&self, image_handle: *mut Handle, map_key: usize) -> Result<()> {
        let status = unsafe {
            (self.bs.exit_boot_services)(image_handle, map_key)
        };

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
