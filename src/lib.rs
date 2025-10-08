// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Rust Integration Library
//!
//! This library provides BSD-2-Clause-Patent licensed bindings and abstractions
//! for UEFI firmware development in Rust.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler))]
#![cfg_attr(not(feature = "std"), feature(lang_items))]
#![allow(clippy::missing_safety_doc)]
#![allow(improper_ctypes_definitions)]
#![allow(clippy::mut_from_ref)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::empty_loop)]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(unused_variables))]
#![cfg_attr(test, allow(clippy::useless_transmute))]
#![cfg_attr(test, allow(clippy::field_reassign_with_default))]
#![cfg_attr(test, allow(clippy::unnecessary_unwrap))]
#![cfg_attr(test, allow(clippy::needless_parens))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod allocator;
pub mod boot_services;
pub mod debug;
pub mod ffi;
pub mod graphics;
pub mod guid;
pub mod intrinsics;
pub mod logger;
pub mod panic_handler;
pub mod protocols;
pub mod runtime_services;
pub mod string;
pub mod system_table;
pub mod tables;

pub use ffi::*;
pub use system_table::SystemTable;

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

/// Panic handler for no_std environment
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler::panic_handler(info)
}

/// Language item for eh_personality
#[cfg(not(feature = "std"))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Compiler built-in memcpy (UEFI provides this via Boot Services)
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

/// Compiler built-in memset (UEFI provides this via Boot Services)
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}

/// Compiler built-in memcmp
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}
