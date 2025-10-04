// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Simple Text Input Protocol

use crate::ffi::*;

/// EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID
pub const SIMPLE_TEXT_INPUT_PROTOCOL_GUID: Guid = Guid::new(
    0x387477c1,
    0x69c7,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// EFI_INPUT_KEY
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputKey {
    pub scan_code: Uint16,
    pub unicode_char: Char16,
}

/// EFI_SIMPLE_TEXT_INPUT_PROTOCOL
#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset: unsafe extern "efiapi" fn(
        this: *mut SimpleTextInputProtocol,
        extended_verification: Boolean,
    ) -> Status,
    pub read_key_stroke: unsafe extern "efiapi" fn(
        this: *mut SimpleTextInputProtocol,
        key: *mut InputKey,
    ) -> Status,
    pub wait_for_key: Event,
}

// Scan codes
pub const SCAN_NULL: u16 = 0x0000;
pub const SCAN_UP: u16 = 0x0001;
pub const SCAN_DOWN: u16 = 0x0002;
pub const SCAN_RIGHT: u16 = 0x0003;
pub const SCAN_LEFT: u16 = 0x0004;
pub const SCAN_HOME: u16 = 0x0005;
pub const SCAN_END: u16 = 0x0006;
pub const SCAN_INSERT: u16 = 0x0007;
pub const SCAN_DELETE: u16 = 0x0008;
pub const SCAN_PAGE_UP: u16 = 0x0009;
pub const SCAN_PAGE_DOWN: u16 = 0x000A;
pub const SCAN_F1: u16 = 0x000B;
pub const SCAN_F2: u16 = 0x000C;
pub const SCAN_F3: u16 = 0x000D;
pub const SCAN_F4: u16 = 0x000E;
pub const SCAN_F5: u16 = 0x000F;
pub const SCAN_F6: u16 = 0x0010;
pub const SCAN_F7: u16 = 0x0011;
pub const SCAN_F8: u16 = 0x0012;
pub const SCAN_F9: u16 = 0x0013;
pub const SCAN_F10: u16 = 0x0014;
pub const SCAN_ESC: u16 = 0x0017;

impl SimpleTextInputProtocol {
    /// Reset the input device
    pub unsafe fn reset(&mut self, extended_verification: bool) -> Status {
        (self.reset)(self, extended_verification as Boolean)
    }

    /// Read a keystroke
    pub unsafe fn read_key_stroke(&mut self) -> Result<InputKey, Status> {
        let mut key = InputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        let status = (self.read_key_stroke)(self, &mut key);
        if status == EFI_SUCCESS {
            Ok(key)
        } else {
            Err(status)
        }
    }
}
