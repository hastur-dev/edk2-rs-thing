// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Raw UEFI FFI types and constants
//!
//! Based on UEFI Specification 2.10

pub mod status;
pub mod table_header;
pub mod types;

pub use status::*;
pub use table_header::*;
pub use types::*;

/// UEFI Handle
#[repr(C)]
pub struct Handle(*mut core::ffi::c_void);

impl Handle {
    pub fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.0
    }
}

/// EFI_GUID structure
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

/// EFI_EVENT
pub type Event = *mut core::ffi::c_void;

/// EFI_TPL (Task Priority Level)
pub type Tpl = usize;

// Standard TPL levels
pub const TPL_APPLICATION: Tpl = 4;
pub const TPL_CALLBACK: Tpl = 8;
pub const TPL_NOTIFY: Tpl = 16;
pub const TPL_HIGH_LEVEL: Tpl = 31;

/// EFI_PHYSICAL_ADDRESS
pub type PhysicalAddress = u64;

/// EFI_VIRTUAL_ADDRESS
pub type VirtualAddress = u64;

/// Memory Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryType {
    ReservedMemoryType = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    ConventionalMemory = 7,
    UnusableMemory = 8,
    ACPIReclaimMemory = 9,
    ACPIMemoryNVS = 10,
    MemoryMappedIO = 11,
    MemoryMappedIOPortSpace = 12,
    PalCode = 13,
    PersistentMemory = 14,
    MaxMemoryType = 15,
}

/// Allocate Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AllocateType {
    AllocateAnyPages = 0,
    AllocateMaxAddress = 1,
    AllocateAddress = 2,
    MaxAllocateType = 3,
}

/// EFI_MEMORY_DESCRIPTOR
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

// Memory attribute bits
pub const EFI_MEMORY_UC: u64 = 0x0000000000000001;
pub const EFI_MEMORY_WC: u64 = 0x0000000000000002;
pub const EFI_MEMORY_WT: u64 = 0x0000000000000004;
pub const EFI_MEMORY_WB: u64 = 0x0000000000000008;
pub const EFI_MEMORY_UCE: u64 = 0x0000000000000010;
pub const EFI_MEMORY_WP: u64 = 0x0000000000001000;
pub const EFI_MEMORY_RP: u64 = 0x0000000000002000;
pub const EFI_MEMORY_XP: u64 = 0x0000000000004000;
pub const EFI_MEMORY_RUNTIME: u64 = 0x8000000000000000;
