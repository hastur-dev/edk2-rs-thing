// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI USB I/O Protocol

use crate::ffi::*;

/// EFI_USB_IO_PROTOCOL_GUID
pub const USB_IO_PROTOCOL_GUID: Guid = Guid::new(
    0x2B2F68D6,
    0x0CD2,
    0x44cf,
    [0x8E, 0x8B, 0xBB, 0xA2, 0x0B, 0x1B, 0x5B, 0x75],
);

/// EFI_USB_DATA_DIRECTION
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UsbDataDirection {
    UsbDataIn = 0,
    UsbDataOut = 1,
    UsbNoData = 2,
}

/// EFI_USB_DEVICE_REQUEST
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct UsbDeviceRequest {
    pub request_type: Uint8,
    pub request: Uint8,
    pub value: Uint16,
    pub index: Uint16,
    pub length: Uint16,
}

/// EFI_USB_DEVICE_DESCRIPTOR
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct UsbDeviceDescriptor {
    pub length: Uint8,
    pub descriptor_type: Uint8,
    pub bcd_usb: Uint16,
    pub device_class: Uint8,
    pub device_sub_class: Uint8,
    pub device_protocol: Uint8,
    pub max_packet_size0: Uint8,
    pub id_vendor: Uint16,
    pub id_product: Uint16,
    pub bcd_device: Uint16,
    pub str_manufacturer: Uint8,
    pub str_product: Uint8,
    pub str_serial_number: Uint8,
    pub num_configurations: Uint8,
}

/// EFI_USB_CONFIG_DESCRIPTOR
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct UsbConfigDescriptor {
    pub length: Uint8,
    pub descriptor_type: Uint8,
    pub total_length: Uint16,
    pub num_interfaces: Uint8,
    pub configuration_value: Uint8,
    pub configuration: Uint8,
    pub attributes: Uint8,
    pub max_power: Uint8,
}

/// EFI_USB_INTERFACE_DESCRIPTOR
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct UsbInterfaceDescriptor {
    pub length: Uint8,
    pub descriptor_type: Uint8,
    pub interface_number: Uint8,
    pub alternate_setting: Uint8,
    pub num_endpoints: Uint8,
    pub interface_class: Uint8,
    pub interface_sub_class: Uint8,
    pub interface_protocol: Uint8,
    pub interface: Uint8,
}

/// EFI_USB_ENDPOINT_DESCRIPTOR
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct UsbEndpointDescriptor {
    pub length: Uint8,
    pub descriptor_type: Uint8,
    pub endpoint_address: Uint8,
    pub attributes: Uint8,
    pub max_packet_size: Uint16,
    pub interval: Uint8,
}

/// EFI_USB_IO_PROTOCOL
#[repr(C)]
pub struct UsbIoProtocol {
    pub usb_control_transfer: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        request: *const UsbDeviceRequest,
        direction: UsbDataDirection,
        timeout: Uint32,
        data: *mut core::ffi::c_void,
        data_length: Uintn,
        status: *mut Uint32,
    ) -> Status,
    pub usb_bulk_transfer: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        endpoint: Uint8,
        data: *mut core::ffi::c_void,
        data_length: *mut Uintn,
        timeout: Uintn,
        status: *mut Uint32,
    ) -> Status,
    pub usb_async_interrupt_transfer: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        endpoint: Uint8,
        is_new_transfer: Boolean,
        polling_interval: Uintn,
        data_length: Uintn,
        callback: *mut core::ffi::c_void,
        context: *mut core::ffi::c_void,
    ) -> Status,
    pub usb_sync_interrupt_transfer: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        endpoint: Uint8,
        data: *mut core::ffi::c_void,
        data_length: *mut Uintn,
        timeout: Uintn,
        status: *mut Uint32,
    ) -> Status,
    pub usb_isochronous_transfer: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        endpoint: Uint8,
        data: *mut core::ffi::c_void,
        data_length: Uintn,
        status: *mut Uint32,
    ) -> Status,
    pub usb_async_isochronous_transfer: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        endpoint: Uint8,
        data: *mut core::ffi::c_void,
        data_length: Uintn,
        callback: *mut core::ffi::c_void,
        context: *mut core::ffi::c_void,
    ) -> Status,
    pub usb_get_device_descriptor: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        descriptor: *mut UsbDeviceDescriptor,
    ) -> Status,
    pub usb_get_config_descriptor: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        descriptor: *mut UsbConfigDescriptor,
    ) -> Status,
    pub usb_get_interface_descriptor: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        descriptor: *mut UsbInterfaceDescriptor,
    ) -> Status,
    pub usb_get_endpoint_descriptor: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        endpoint_index: Uint8,
        descriptor: *mut UsbEndpointDescriptor,
    ) -> Status,
    pub usb_get_string_descriptor: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        lang_id: Uint16,
        string_id: Uint8,
        string: *mut *mut Char16,
    ) -> Status,
    pub usb_get_supported_languages: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
        lang_id_table: *mut *mut Uint16,
        table_size: *mut Uint16,
    ) -> Status,
    pub usb_port_reset: unsafe extern "efiapi" fn(
        this: *mut UsbIoProtocol,
    ) -> Status,
}

impl UsbIoProtocol {
    /// Perform a USB control transfer
    pub unsafe fn control_transfer(
        &mut self,
        request: &UsbDeviceRequest,
        direction: UsbDataDirection,
        timeout: u32,
        data: Option<&mut [u8]>,
    ) -> Result<u32, Status> {
        let mut usb_status = 0u32;
        let (data_ptr, data_len) = match data {
            Some(buf) => (buf.as_mut_ptr() as *mut core::ffi::c_void, buf.len()),
            None => (core::ptr::null_mut(), 0),
        };

        let status = (self.usb_control_transfer)(
            self,
            request,
            direction,
            timeout,
            data_ptr,
            data_len,
            &mut usb_status,
        );

        if status == EFI_SUCCESS {
            Ok(usb_status)
        } else {
            Err(status)
        }
    }

    /// Perform a USB bulk transfer
    pub unsafe fn bulk_transfer(
        &mut self,
        endpoint: u8,
        data: &mut [u8],
        timeout: usize,
    ) -> Result<(usize, u32), Status> {
        let mut data_length = data.len();
        let mut usb_status = 0u32;

        let status = (self.usb_bulk_transfer)(
            self,
            endpoint,
            data.as_mut_ptr() as *mut core::ffi::c_void,
            &mut data_length,
            timeout,
            &mut usb_status,
        );

        if status == EFI_SUCCESS {
            Ok((data_length, usb_status))
        } else {
            Err(status)
        }
    }

    /// Get device descriptor
    pub unsafe fn get_device_descriptor(&mut self) -> Result<UsbDeviceDescriptor, Status> {
        let mut descriptor = core::mem::zeroed();
        let status = (self.usb_get_device_descriptor)(self, &mut descriptor);

        if status == EFI_SUCCESS {
            Ok(descriptor)
        } else {
            Err(status)
        }
    }

    /// Get configuration descriptor
    pub unsafe fn get_config_descriptor(&mut self) -> Result<UsbConfigDescriptor, Status> {
        let mut descriptor = core::mem::zeroed();
        let status = (self.usb_get_config_descriptor)(self, &mut descriptor);

        if status == EFI_SUCCESS {
            Ok(descriptor)
        } else {
            Err(status)
        }
    }

    /// Get interface descriptor
    pub unsafe fn get_interface_descriptor(&mut self) -> Result<UsbInterfaceDescriptor, Status> {
        let mut descriptor = core::mem::zeroed();
        let status = (self.usb_get_interface_descriptor)(self, &mut descriptor);

        if status == EFI_SUCCESS {
            Ok(descriptor)
        } else {
            Err(status)
        }
    }

    /// Reset USB port
    pub unsafe fn port_reset(&mut self) -> Status {
        (self.usb_port_reset)(self)
    }
}

// USB Request Types
pub const USB_REQ_TYPE_STANDARD: u8 = 0x00;
pub const USB_REQ_TYPE_CLASS: u8 = 0x20;
pub const USB_REQ_TYPE_VENDOR: u8 = 0x40;

// USB Standard Requests
pub const USB_REQ_GET_STATUS: u8 = 0x00;
pub const USB_REQ_CLEAR_FEATURE: u8 = 0x01;
pub const USB_REQ_SET_FEATURE: u8 = 0x03;
pub const USB_REQ_SET_ADDRESS: u8 = 0x05;
pub const USB_REQ_GET_DESCRIPTOR: u8 = 0x06;
pub const USB_REQ_SET_DESCRIPTOR: u8 = 0x07;
pub const USB_REQ_GET_CONFIGURATION: u8 = 0x08;
pub const USB_REQ_SET_CONFIGURATION: u8 = 0x09;
pub const USB_REQ_GET_INTERFACE: u8 = 0x0A;
pub const USB_REQ_SET_INTERFACE: u8 = 0x0B;
pub const USB_REQ_SYNCH_FRAME: u8 = 0x0C;

// USB Descriptor Types
pub const USB_DESC_TYPE_DEVICE: u8 = 0x01;
pub const USB_DESC_TYPE_CONFIG: u8 = 0x02;
pub const USB_DESC_TYPE_STRING: u8 = 0x03;
pub const USB_DESC_TYPE_INTERFACE: u8 = 0x04;
pub const USB_DESC_TYPE_ENDPOINT: u8 = 0x05;
