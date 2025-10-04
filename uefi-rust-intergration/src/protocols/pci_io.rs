// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI PCI I/O Protocol

use crate::ffi::*;

/// EFI_PCI_IO_PROTOCOL_GUID
pub const PCI_IO_PROTOCOL_GUID: Guid = Guid::new(
    0x4cf5b200,
    0x68b8,
    0x4ca5,
    [0x9e, 0xec, 0xb2, 0x3e, 0x3f, 0x50, 0x02, 0x9a],
);

/// EFI_PCI_IO_PROTOCOL_WIDTH
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PciIoWidth {
    PciIoWidthUint8 = 0,
    PciIoWidthUint16 = 1,
    PciIoWidthUint32 = 2,
    PciIoWidthUint64 = 3,
    PciIoWidthFifoUint8 = 4,
    PciIoWidthFifoUint16 = 5,
    PciIoWidthFifoUint32 = 6,
    PciIoWidthFifoUint64 = 7,
    PciIoWidthFillUint8 = 8,
    PciIoWidthFillUint16 = 9,
    PciIoWidthFillUint32 = 10,
    PciIoWidthFillUint64 = 11,
    PciIoWidthMaximum = 12,
}

/// EFI_PCI_IO_PROTOCOL_ATTRIBUTE_OPERATION
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PciIoAttributeOperation {
    EfiPciIoAttributeOperationGet = 0,
    EfiPciIoAttributeOperationSet = 1,
    EfiPciIoAttributeOperationEnable = 2,
    EfiPciIoAttributeOperationDisable = 3,
    EfiPciIoAttributeOperationSupported = 4,
    EfiPciIoAttributeOperationMaximum = 5,
}

// PCI attributes
pub const EFI_PCI_IO_ATTRIBUTE_ISA_MOTHERBOARD_IO: u64 = 0x0001;
pub const EFI_PCI_IO_ATTRIBUTE_ISA_IO: u64 = 0x0002;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_PALETTE_IO: u64 = 0x0004;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_MEMORY: u64 = 0x0008;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_IO: u64 = 0x0010;
pub const EFI_PCI_IO_ATTRIBUTE_IDE_PRIMARY_IO: u64 = 0x0020;
pub const EFI_PCI_IO_ATTRIBUTE_IDE_SECONDARY_IO: u64 = 0x0040;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_WRITE_COMBINE: u64 = 0x0080;
pub const EFI_PCI_IO_ATTRIBUTE_IO: u64 = 0x0100;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY: u64 = 0x0200;
pub const EFI_PCI_IO_ATTRIBUTE_BUS_MASTER: u64 = 0x0400;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_CACHED: u64 = 0x0800;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_DISABLE: u64 = 0x1000;
pub const EFI_PCI_IO_ATTRIBUTE_EMBEDDED_DEVICE: u64 = 0x2000;
pub const EFI_PCI_IO_ATTRIBUTE_EMBEDDED_ROM: u64 = 0x4000;
pub const EFI_PCI_IO_ATTRIBUTE_DUAL_ADDRESS_CYCLE: u64 = 0x8000;
pub const EFI_PCI_IO_ATTRIBUTE_ISA_IO_16: u64 = 0x10000;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_PALETTE_IO_16: u64 = 0x20000;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_IO_16: u64 = 0x40000;

type PciIoProtocolIoAccess = unsafe extern "efiapi" fn(
    this: *mut PciIoProtocol,
    width: PciIoWidth,
    offset: Uint64,
    count: Uintn,
    buffer: *mut core::ffi::c_void,
) -> Status;

/// EFI_PCI_IO_PROTOCOL_ACCESS
#[repr(C)]
pub struct PciIoProtocolAccess {
    pub read: PciIoProtocolIoAccess,
    pub write: PciIoProtocolIoAccess,
}

/// EFI_PCI_IO_PROTOCOL_CONFIG_ACCESS
#[repr(C)]
pub struct PciIoProtocolConfigAccess {
    pub read: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        width: PciIoWidth,
        offset: Uint32,
        count: Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub write: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        width: PciIoWidth,
        offset: Uint32,
        count: Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
}

/// EFI_PCI_IO_PROTOCOL
#[repr(C)]
pub struct PciIoProtocol {
    pub poll_mem: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        width: PciIoWidth,
        offset: Uint64,
        mask: Uint64,
        value: Uint64,
        delay: Uint64,
        result: *mut Uint64,
    ) -> Status,
    pub poll_io: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        width: PciIoWidth,
        offset: Uint64,
        mask: Uint64,
        value: Uint64,
        delay: Uint64,
        result: *mut Uint64,
    ) -> Status,
    pub mem: PciIoProtocolAccess,
    pub io: PciIoProtocolAccess,
    pub pci: PciIoProtocolConfigAccess,
    pub copy_mem: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        width: PciIoWidth,
        dest_offset: Uint64,
        src_offset: Uint64,
        count: Uintn,
    ) -> Status,
    pub map: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        operation: Uint32,
        host_address: *mut core::ffi::c_void,
        number_of_bytes: *mut Uintn,
        device_address: *mut PhysicalAddress,
        mapping: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub unmap: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        mapping: *mut core::ffi::c_void,
    ) -> Status,
    pub allocate_buffer: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        alloc_type: Uint32,
        memory_type: MemoryType,
        pages: Uintn,
        host_address: *mut *mut core::ffi::c_void,
        attributes: Uint64,
    ) -> Status,
    pub free_buffer: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        pages: Uintn,
        host_address: *mut core::ffi::c_void,
    ) -> Status,
    pub flush: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
    ) -> Status,
    pub get_location: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        segment_number: *mut Uintn,
        bus_number: *mut Uintn,
        device_number: *mut Uintn,
        function_number: *mut Uintn,
    ) -> Status,
    pub attributes: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        operation: PciIoAttributeOperation,
        attributes: Uint64,
        result: *mut Uint64,
    ) -> Status,
    pub get_bar_attributes: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        bar_index: Uint8,
        supports: *mut Uint64,
        resources: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub set_bar_attributes: unsafe extern "efiapi" fn(
        this: *mut PciIoProtocol,
        attributes: Uint64,
        bar_index: Uint8,
        offset: *mut Uint64,
        length: *mut Uint64,
    ) -> Status,
    pub rom_size: Uint64,
    pub rom_image: *mut core::ffi::c_void,
}

impl PciIoProtocol {
    /// Get PCI location
    pub unsafe fn get_location(&mut self) -> Result<(usize, usize, usize, usize), Status> {
        let mut segment = 0;
        let mut bus = 0;
        let mut device = 0;
        let mut function = 0;
        let status = (self.get_location)(self, &mut segment, &mut bus, &mut device, &mut function);
        if status == EFI_SUCCESS {
            Ok((segment, bus, device, function))
        } else {
            Err(status)
        }
    }

    /// Read from PCI configuration space
    pub unsafe fn pci_read(
        &mut self,
        width: PciIoWidth,
        offset: u32,
        buffer: &mut [u8],
    ) -> Status {
        let count = buffer.len() / (width as usize + 1);
        (self.pci.read)(self, width, offset, count, buffer.as_mut_ptr() as *mut _)
    }

    /// Write to PCI configuration space
    pub unsafe fn pci_write(
        &mut self,
        width: PciIoWidth,
        offset: u32,
        buffer: &[u8],
    ) -> Status {
        let count = buffer.len() / (width as usize + 1);
        (self.pci.write)(self, width, offset, count, buffer.as_ptr() as *mut _)
    }
}
