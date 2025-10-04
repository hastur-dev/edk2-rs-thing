// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI String Utilities - UCS-2/UTF-16 conversion and manipulation

use crate::ffi::*;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Convert a Rust string slice to a null-terminated UCS-2 (Char16) vector
pub fn str_to_ucs2(s: &str) -> Vec<Char16> {
    let mut result: Vec<Char16> = s.encode_utf16().collect();
    result.push(0); // Null terminator
    result
}

/// Convert a Rust string to a null-terminated UCS-2 (Char16) vector
pub fn string_to_ucs2(s: String) -> Vec<Char16> {
    str_to_ucs2(&s)
}

/// Convert a null-terminated UCS-2 (Char16) pointer to a Rust String
///
/// # Safety
/// The pointer must point to a valid null-terminated UCS-2 string
pub unsafe fn ucs2_to_string(ptr: *const Char16) -> Result<String, core::char::DecodeUtf16Error> {
    if ptr.is_null() {
        return Ok(String::new());
    }

    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }

    let slice = core::slice::from_raw_parts(ptr, len);
    char::decode_utf16(slice.iter().copied())
        .collect::<Result<String, _>>()
}

/// Get the length of a null-terminated UCS-2 string
///
/// # Safety
/// The pointer must point to a valid null-terminated UCS-2 string
pub unsafe fn ucs2_strlen(ptr: *const Char16) -> usize {
    if ptr.is_null() {
        return 0;
    }

    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    len
}

/// Compare two null-terminated UCS-2 strings
///
/// # Safety
/// Both pointers must point to valid null-terminated UCS-2 strings
pub unsafe fn ucs2_strcmp(s1: *const Char16, s2: *const Char16) -> i32 {
    if s1.is_null() && s2.is_null() {
        return 0;
    }
    if s1.is_null() {
        return -1;
    }
    if s2.is_null() {
        return 1;
    }

    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);

        if c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }

        if c1 == 0 {
            return 0;
        }

        i += 1;
    }
}

/// Copy a null-terminated UCS-2 string
///
/// # Safety
/// - src must point to a valid null-terminated UCS-2 string
/// - dest must have enough space to hold the string including null terminator
pub unsafe fn ucs2_strcpy(dest: *mut Char16, src: *const Char16) {
    if src.is_null() || dest.is_null() {
        return;
    }

    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// Copy at most n characters from a null-terminated UCS-2 string
///
/// # Safety
/// - src must point to a valid UCS-2 string
/// - dest must have enough space for n characters
pub unsafe fn ucs2_strncpy(dest: *mut Char16, src: *const Char16, n: usize) {
    if src.is_null() || dest.is_null() || n == 0 {
        return;
    }

    let mut i = 0;
    while i < n {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }

    // Ensure null termination
    if i < n {
        while i < n {
            *dest.add(i) = 0;
            i += 1;
        }
    }
}

/// Concatenate two null-terminated UCS-2 strings
///
/// # Safety
/// - dest must point to a valid null-terminated UCS-2 string with enough space
/// - src must point to a valid null-terminated UCS-2 string
pub unsafe fn ucs2_strcat(dest: *mut Char16, src: *const Char16) {
    if src.is_null() || dest.is_null() {
        return;
    }

    let dest_len = ucs2_strlen(dest);
    ucs2_strcpy(dest.add(dest_len), src);
}

/// Helper macro to create a null-terminated UCS-2 string literal at compile time
#[macro_export]
macro_rules! ucs2 {
    ($s:expr) => {{
        const fn encode_ucs2(s: &str) -> ([u16; $s.len() + 1], usize) {
            let bytes = s.as_bytes();
            let mut result = [0u16; $s.len() + 1];
            let mut i = 0;
            while i < bytes.len() {
                result[i] = bytes[i] as u16;
                i += 1;
            }
            result[$s.len()] = 0;
            (result, $s.len() + 1)
        }

        const DATA: ([u16; $s.len() + 1], usize) = encode_ucs2($s);
        &DATA.0[..DATA.1]
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_ucs2() {
        let s = "Hello";
        let ucs2 = str_to_ucs2(s);
        assert_eq!(ucs2.len(), 6); // 5 chars + null terminator
        assert_eq!(ucs2[0], 'H' as u16);
        assert_eq!(ucs2[5], 0);
    }

    #[test]
    fn test_ucs2_to_string() {
        let ucs2: Vec<Char16> = vec!['H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0];
        let s = unsafe { ucs2_to_string(ucs2.as_ptr()).unwrap() };
        assert_eq!(s, "Hello");
    }

    #[test]
    fn test_ucs2_strlen() {
        let ucs2: Vec<Char16> = vec!['H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0];
        let len = unsafe { ucs2_strlen(ucs2.as_ptr()) };
        assert_eq!(len, 5);
    }

    #[test]
    fn test_ucs2_strcmp() {
        let s1: Vec<Char16> = vec!['H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0];
        let s2: Vec<Char16> = vec!['H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0];
        let s3: Vec<Char16> = vec!['W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, 0];

        let cmp1 = unsafe { ucs2_strcmp(s1.as_ptr(), s2.as_ptr()) };
        let cmp2 = unsafe { ucs2_strcmp(s1.as_ptr(), s3.as_ptr()) };

        assert_eq!(cmp1, 0);
        assert!(cmp2 < 0);
    }
}
