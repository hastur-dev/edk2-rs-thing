// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Simple Text Output Protocol

use crate::ffi::*;

/// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID
pub const SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: Guid = Guid::new(
    0x387477c2,
    0x69c7,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// SIMPLE_TEXT_OUTPUT_MODE
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SimpleTextOutputMode {
    pub max_mode: Int32,
    pub mode: Int32,
    pub attribute: Int32,
    pub cursor_column: Int32,
    pub cursor_row: Int32,
    pub cursor_visible: Boolean,
}

/// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        extended_verification: Boolean,
    ) -> Status,
    pub output_string: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        string: *const Char16,
    ) -> Status,
    pub test_string: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        string: *const Char16,
    ) -> Status,
    pub query_mode: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        mode_number: Uintn,
        columns: *mut Uintn,
        rows: *mut Uintn,
    ) -> Status,
    pub set_mode: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        mode_number: Uintn,
    ) -> Status,
    pub set_attribute: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        attribute: Uintn,
    ) -> Status,
    pub clear_screen: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
    ) -> Status,
    pub set_cursor_position: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        column: Uintn,
        row: Uintn,
    ) -> Status,
    pub enable_cursor: unsafe extern "efiapi" fn(
        this: *mut SimpleTextOutputProtocol,
        visible: Boolean,
    ) -> Status,
    pub mode: *mut SimpleTextOutputMode,
}

// Text attributes
pub const EFI_BLACK: usize = 0x00;
pub const EFI_BLUE: usize = 0x01;
pub const EFI_GREEN: usize = 0x02;
pub const EFI_CYAN: usize = 0x03;
pub const EFI_RED: usize = 0x04;
pub const EFI_MAGENTA: usize = 0x05;
pub const EFI_BROWN: usize = 0x06;
pub const EFI_LIGHTGRAY: usize = 0x07;
pub const EFI_BRIGHT: usize = 0x08;
pub const EFI_DARKGRAY: usize = 0x08;
pub const EFI_LIGHTBLUE: usize = 0x09;
pub const EFI_LIGHTGREEN: usize = 0x0A;
pub const EFI_LIGHTCYAN: usize = 0x0B;
pub const EFI_LIGHTRED: usize = 0x0C;
pub const EFI_LIGHTMAGENTA: usize = 0x0D;
pub const EFI_YELLOW: usize = 0x0E;
pub const EFI_WHITE: usize = 0x0F;

pub const EFI_BACKGROUND_BLACK: usize = 0x00;
pub const EFI_BACKGROUND_BLUE: usize = 0x10;
pub const EFI_BACKGROUND_GREEN: usize = 0x20;
pub const EFI_BACKGROUND_CYAN: usize = 0x30;
pub const EFI_BACKGROUND_RED: usize = 0x40;
pub const EFI_BACKGROUND_MAGENTA: usize = 0x50;
pub const EFI_BACKGROUND_BROWN: usize = 0x60;
pub const EFI_BACKGROUND_LIGHTGRAY: usize = 0x70;

#[inline]
pub const fn efi_text_attr(foreground: usize, background: usize) -> usize {
    foreground | background
}

impl SimpleTextOutputProtocol {
    /// Reset the output device
    pub unsafe fn reset(&mut self, extended_verification: bool) -> Status {
        (self.reset)(self, extended_verification as Boolean)
    }

    /// Output a string
    pub unsafe fn output_string(&mut self, string: *const Char16) -> Status {
        (self.output_string)(self, string)
    }

    /// Test if a string can be output
    pub unsafe fn test_string(&mut self, string: *const Char16) -> Status {
        (self.test_string)(self, string)
    }

    /// Query mode information
    pub unsafe fn query_mode(&mut self, mode_number: usize) -> Result<(usize, usize), Status> {
        let mut columns = 0;
        let mut rows = 0;
        let status = (self.query_mode)(self, mode_number, &mut columns, &mut rows);
        if status == EFI_SUCCESS {
            Ok((columns, rows))
        } else {
            Err(status)
        }
    }

    /// Set mode
    pub unsafe fn set_mode(&mut self, mode_number: usize) -> Status {
        (self.set_mode)(self, mode_number)
    }

    /// Set attribute
    pub unsafe fn set_attribute(&mut self, attribute: usize) -> Status {
        (self.set_attribute)(self, attribute)
    }

    /// Clear screen
    pub unsafe fn clear_screen(&mut self) -> Status {
        (self.clear_screen)(self)
    }

    /// Set cursor position
    pub unsafe fn set_cursor_position(&mut self, column: usize, row: usize) -> Status {
        (self.set_cursor_position)(self, column, row)
    }

    /// Enable cursor
    pub unsafe fn enable_cursor(&mut self, visible: bool) -> Status {
        (self.enable_cursor)(self, visible as Boolean)
    }
}
