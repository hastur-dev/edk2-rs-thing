// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Global Allocator implementation

use crate::boot_services::BootServices;
use crate::ffi::*;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// Global reference to Boot Services (set during initialization)
static mut BOOT_SERVICES: Option<&'static BootServices> = None;

/// Initialize the allocator with Boot Services
///
/// # Safety
/// Must be called once during UEFI application initialization
pub unsafe fn init_allocator(boot_services: &'static BootServices) {
    BOOT_SERVICES = Some(boot_services);
}

/// UEFI Global Allocator
pub struct UefiAllocator;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(bs) = BOOT_SERVICES {
            let mut buffer: *mut core::ffi::c_void = null_mut();
            let size = layout.size();
            let align = layout.align();

            // UEFI AllocatePool returns 8-byte aligned memory
            // For larger alignments, we need to allocate extra space
            if align <= 8 {
                // Standard allocation
                let status = (bs.allocate_pool)(MemoryType::LoaderData, size, &mut buffer);

                if status == EFI_SUCCESS && !buffer.is_null() {
                    buffer as *mut u8
                } else {
                    null_mut()
                }
            } else {
                // Over-allocate to handle alignment
                let total_size = size + align + core::mem::size_of::<usize>();

                let status = (bs.allocate_pool)(MemoryType::LoaderData, total_size, &mut buffer);

                if status != EFI_SUCCESS || buffer.is_null() {
                    return null_mut();
                }

                let base_addr = buffer as usize;

                // Calculate aligned address
                let offset = core::mem::size_of::<usize>();
                let data_start = base_addr + offset;
                let aligned_addr = (data_start + align - 1) & !(align - 1);

                // Store the original pointer before the aligned address
                let header = (aligned_addr - core::mem::size_of::<usize>()) as *mut usize;
                *header = base_addr;

                aligned_addr as *mut u8
            }
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(bs) = BOOT_SERVICES {
            let align = layout.align();

            if align <= 8 {
                // Standard deallocation
                let _ = (bs.free_pool)(ptr as *mut core::ffi::c_void);
            } else {
                // Retrieve original pointer from header
                let header = (ptr as usize - core::mem::size_of::<usize>()) as *const usize;
                let original_ptr = *header;
                let _ = (bs.free_pool)(original_ptr as *mut core::ffi::c_void);
            }
        }
    }
}

#[cfg(not(feature = "std"))]
#[global_allocator]
static ALLOCATOR: UefiAllocator = UefiAllocator;

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}
