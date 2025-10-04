// SPDX-License-Identifier: BSD-2-Clause-Patent
//! GUID Management Utilities

use crate::ffi::Guid;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Helper macro to define a GUID constant
#[macro_export]
macro_rules! guid {
    ($name:ident = $d1:expr, $d2:expr, $d3:expr, [$d4_0:expr, $d4_1:expr, $d4_2:expr, $d4_3:expr, $d4_4:expr, $d4_5:expr, $d4_6:expr, $d4_7:expr]) => {
        pub const $name: $crate::ffi::Guid = $crate::ffi::Guid::new(
            $d1, $d2, $d3,
            [$d4_0, $d4_1, $d4_2, $d4_3, $d4_4, $d4_5, $d4_6, $d4_7],
        );
    };
}

impl core::fmt::Display for Guid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

impl Guid {
    /// Parse a GUID from a string
    /// Format: "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 36 {
            return None;
        }

        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 5 {
            return None;
        }

        let data1 = u32::from_str_radix(parts[0], 16).ok()?;
        let data2 = u16::from_str_radix(parts[1], 16).ok()?;
        let data3 = u16::from_str_radix(parts[2], 16).ok()?;

        let mut data4 = [0u8; 8];
        let part3 = parts[3];
        if part3.len() != 4 {
            return None;
        }
        data4[0] = u8::from_str_radix(&part3[0..2], 16).ok()?;
        data4[1] = u8::from_str_radix(&part3[2..4], 16).ok()?;

        let part4 = parts[4];
        if part4.len() != 12 {
            return None;
        }
        for i in 0..6 {
            data4[i + 2] = u8::from_str_radix(&part4[i * 2..i * 2 + 2], 16).ok()?;
        }

        Some(Guid::new(data1, data2, data3, data4))
    }

    /// Check if this GUID is null (all zeros)
    pub fn is_null(&self) -> bool {
        self.data1 == 0 && self.data2 == 0 && self.data3 == 0 && self.data4 == [0; 8]
    }

    /// Create a null GUID
    pub const fn null() -> Self {
        Guid::new(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_display() {
        let guid = Guid::new(
            0x12345678,
            0x1234,
            0x5678,
            [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
        );
        let s = format!("{}", guid);
        assert_eq!(s, "12345678-1234-5678-1234-56789ABCDEF0");
    }

    #[test]
    fn test_guid_from_str() {
        let s = "12345678-1234-5678-1234-56789ABCDEF0";
        let guid = Guid::from_str(s).unwrap();
        assert_eq!(guid.data1, 0x12345678);
        assert_eq!(guid.data2, 0x1234);
        assert_eq!(guid.data3, 0x5678);
        assert_eq!(
            guid.data4,
            [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]
        );
    }

    #[test]
    fn test_guid_null() {
        let guid = Guid::null();
        assert!(guid.is_null());

        let guid2 = Guid::new(1, 0, 0, [0; 8]);
        assert!(!guid2.is_null());
    }
}
