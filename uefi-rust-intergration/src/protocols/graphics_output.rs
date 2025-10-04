// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Graphics Output Protocol

use crate::ffi::*;

/// EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID
pub const GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid::new(
    0x9042a9de,
    0x23dc,
    0x4a38,
    [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);

/// EFI_GRAPHICS_PIXEL_FORMAT
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GraphicsPixelFormat {
    PixelRedGreenBlueReserved8BitPerColor = 0,
    PixelBlueGreenRedReserved8BitPerColor = 1,
    PixelBitMask = 2,
    PixelBltOnly = 3,
    PixelFormatMax = 4,
}

/// EFI_PIXEL_BITMASK
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PixelBitmask {
    pub red_mask: Uint32,
    pub green_mask: Uint32,
    pub blue_mask: Uint32,
    pub reserved_mask: Uint32,
}

/// EFI_GRAPHICS_OUTPUT_MODE_INFORMATION
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GraphicsOutputModeInformation {
    pub version: Uint32,
    pub horizontal_resolution: Uint32,
    pub vertical_resolution: Uint32,
    pub pixel_format: GraphicsPixelFormat,
    pub pixel_information: PixelBitmask,
    pub pixels_per_scan_line: Uint32,
}

/// EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE
#[repr(C)]
pub struct GraphicsOutputProtocolMode {
    pub max_mode: Uint32,
    pub mode: Uint32,
    pub info: *mut GraphicsOutputModeInformation,
    pub size_of_info: Uintn,
    pub frame_buffer_base: PhysicalAddress,
    pub frame_buffer_size: Uintn,
}

/// EFI_GRAPHICS_OUTPUT_BLT_PIXEL
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GraphicsOutputBltPixel {
    pub blue: Uint8,
    pub green: Uint8,
    pub red: Uint8,
    pub reserved: Uint8,
}

/// EFI_GRAPHICS_OUTPUT_BLT_OPERATION
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GraphicsOutputBltOperation {
    EfiBltVideoFill = 0,
    EfiBltVideoToBltBuffer = 1,
    EfiBltBufferToVideo = 2,
    EfiBltVideoToVideo = 3,
    EfiGraphicsOutputBltOperationMax = 4,
}

/// EFI_GRAPHICS_OUTPUT_PROTOCOL
#[repr(C)]
pub struct GraphicsOutputProtocol {
    pub query_mode: unsafe extern "efiapi" fn(
        this: *mut GraphicsOutputProtocol,
        mode_number: Uint32,
        size_of_info: *mut Uintn,
        info: *mut *mut GraphicsOutputModeInformation,
    ) -> Status,
    pub set_mode: unsafe extern "efiapi" fn(
        this: *mut GraphicsOutputProtocol,
        mode_number: Uint32,
    ) -> Status,
    pub blt: unsafe extern "efiapi" fn(
        this: *mut GraphicsOutputProtocol,
        blt_buffer: *mut GraphicsOutputBltPixel,
        blt_operation: GraphicsOutputBltOperation,
        source_x: Uintn,
        source_y: Uintn,
        destination_x: Uintn,
        destination_y: Uintn,
        width: Uintn,
        height: Uintn,
        delta: Uintn,
    ) -> Status,
    pub mode: *mut GraphicsOutputProtocolMode,
}

impl GraphicsOutputProtocol {
    /// Query mode information
    pub unsafe fn query_mode(
        &mut self,
        mode_number: u32,
    ) -> Result<(*mut GraphicsOutputModeInformation, usize), Status> {
        let mut size_of_info = 0;
        let mut info = core::ptr::null_mut();
        let status = (self.query_mode)(self, mode_number, &mut size_of_info, &mut info);
        if status == EFI_SUCCESS {
            Ok((info, size_of_info))
        } else {
            Err(status)
        }
    }

    /// Set mode
    pub unsafe fn set_mode(&mut self, mode_number: u32) -> Status {
        (self.set_mode)(self, mode_number)
    }

    /// Block transfer
    pub unsafe fn blt(
        &mut self,
        blt_buffer: *mut GraphicsOutputBltPixel,
        blt_operation: GraphicsOutputBltOperation,
        source_x: usize,
        source_y: usize,
        destination_x: usize,
        destination_y: usize,
        width: usize,
        height: usize,
        delta: usize,
    ) -> Status {
        (self.blt)(
            self,
            blt_buffer,
            blt_operation,
            source_x,
            source_y,
            destination_x,
            destination_y,
            width,
            height,
            delta,
        )
    }

    /// Get current mode information
    pub unsafe fn current_mode_info(&self) -> Option<&GraphicsOutputModeInformation> {
        if self.mode.is_null() {
            None
        } else {
            let mode = &*self.mode;
            if mode.info.is_null() {
                None
            } else {
                Some(&*mode.info)
            }
        }
    }

    /// Get frame buffer base address
    pub unsafe fn frame_buffer_base(&self) -> Option<PhysicalAddress> {
        if self.mode.is_null() {
            None
        } else {
            Some((*self.mode).frame_buffer_base)
        }
    }

    /// Get frame buffer size
    pub unsafe fn frame_buffer_size(&self) -> Option<usize> {
        if self.mode.is_null() {
            None
        } else {
            Some((*self.mode).frame_buffer_size)
        }
    }
}
