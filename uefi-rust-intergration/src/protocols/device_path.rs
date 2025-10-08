// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Device Path Protocol

use crate::ffi::*;

/// EFI_DEVICE_PATH_PROTOCOL_GUID
pub const DEVICE_PATH_PROTOCOL_GUID: Guid = Guid::new(
    0x09576e91,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// EFI_DEVICE_PATH_PROTOCOL
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DevicePathProtocol {
    pub path_type: Uint8,
    pub sub_type: Uint8,
    pub length: [Uint8; 2],
}

// Device path types
pub const HARDWARE_DEVICE_PATH: u8 = 0x01;
pub const ACPI_DEVICE_PATH: u8 = 0x02;
pub const MESSAGING_DEVICE_PATH: u8 = 0x03;
pub const MEDIA_DEVICE_PATH: u8 = 0x04;
pub const BBS_DEVICE_PATH: u8 = 0x05;
pub const END_DEVICE_PATH_TYPE: u8 = 0x7F;

// End device path sub-types
pub const END_ENTIRE_DEVICE_PATH_SUBTYPE: u8 = 0xFF;
pub const END_INSTANCE_DEVICE_PATH_SUBTYPE: u8 = 0x01;

// Hardware device path sub-types
pub const HW_PCI_DP: u8 = 0x01;
pub const HW_PCCARD_DP: u8 = 0x02;
pub const HW_MEMMAP_DP: u8 = 0x03;
pub const HW_VENDOR_DP: u8 = 0x04;
pub const HW_CONTROLLER_DP: u8 = 0x05;

// ACPI device path sub-types
pub const ACPI_DP: u8 = 0x01;
pub const ACPI_EXTENDED_DP: u8 = 0x02;
pub const ACPI_ADR_DP: u8 = 0x03;

// Messaging device path sub-types
pub const MSG_ATAPI_DP: u8 = 0x01;
pub const MSG_SCSI_DP: u8 = 0x02;
pub const MSG_FIBRECHANNEL_DP: u8 = 0x03;
pub const MSG_USB_DP: u8 = 0x05;
pub const MSG_MAC_ADDR_DP: u8 = 0x0b;
pub const MSG_IPV4_DP: u8 = 0x0c;
pub const MSG_IPV6_DP: u8 = 0x0d;
pub const MSG_UART_DP: u8 = 0x0e;
pub const MSG_VENDOR_DP: u8 = 0x0a;
pub const MSG_SATA_DP: u8 = 0x12;

// Media device path sub-types
pub const MEDIA_HARDDRIVE_DP: u8 = 0x01;
pub const MEDIA_CDROM_DP: u8 = 0x02;
pub const MEDIA_VENDOR_DP: u8 = 0x03;
pub const MEDIA_FILEPATH_DP: u8 = 0x04;
pub const MEDIA_PROTOCOL_DP: u8 = 0x05;
pub const MEDIA_PIWG_FW_FILE_DP: u8 = 0x06;
pub const MEDIA_PIWG_FW_VOL_DP: u8 = 0x07;

impl DevicePathProtocol {
    /// Get the length of this device path node
    pub fn length(&self) -> u16 {
        u16::from_le_bytes(self.length)
    }

    /// Check if this is an end node
    pub fn is_end(&self) -> bool {
        self.path_type == END_DEVICE_PATH_TYPE
    }

    /// Check if this is the end of the entire device path
    pub fn is_end_entire(&self) -> bool {
        self.path_type == END_DEVICE_PATH_TYPE && self.sub_type == END_ENTIRE_DEVICE_PATH_SUBTYPE
    }

    /// Get next device path node
    pub unsafe fn next(&self) -> Option<&DevicePathProtocol> {
        if self.is_end() {
            None
        } else {
            let length = self.length() as usize;
            if length < core::mem::size_of::<DevicePathProtocol>() {
                None
            } else {
                let ptr = (self as *const _ as *const u8).add(length);
                Some(&*(ptr as *const DevicePathProtocol))
            }
        }
    }
}

/// PCI Device Path
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PciDevicePath {
    pub header: DevicePathProtocol,
    pub function: Uint8,
    pub device: Uint8,
}

/// File Path Device Path
#[repr(C)]
pub struct FilePathDevicePath {
    pub header: DevicePathProtocol,
    // Followed by PathName[variable length]
}

/// Hard Drive Device Path
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HardDriveDevicePath {
    pub header: DevicePathProtocol,
    pub partition_number: Uint32,
    pub partition_start: Uint64,
    pub partition_size: Uint64,
    pub signature: [Uint8; 16],
    pub mbr_type: Uint8,
    pub signature_type: Uint8,
}
