// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Runtime Services Integration Tests

#![cfg(test)]

use uefi_rust_intergration::ffi::*;
use uefi_rust_intergration::runtime_services::*;

#[test]
fn test_time_structure_size() {
    // Time structure should be properly sized
    assert_eq!(core::mem::size_of::<Time>(), 16);
}

#[test]
fn test_time_capabilities_size() {
    assert_eq!(core::mem::size_of::<TimeCapabilities>(), 12);
}

#[test]
fn test_reset_type_values() {
    assert_eq!(ResetType::EfiResetCold as u32, 0);
    assert_eq!(ResetType::EfiResetWarm as u32, 1);
    assert_eq!(ResetType::EfiResetShutdown as u32, 2);
    assert_eq!(ResetType::EfiResetPlatformSpecific as u32, 3);
}

#[test]
fn test_runtime_services_signature() {
    // Just verify the constant is correct
    assert_eq!(EFI_RUNTIME_SERVICES_SIGNATURE, 0x56524553544e5552);
}

#[test]
fn test_time_creation() {
    let time = Time {
        year: 2025,
        month: 10,
        day: 3,
        hour: 12,
        minute: 30,
        second: 45,
        pad1: 0,
        nanosecond: 0,
        time_zone: 0,
        daylight: 0,
        pad2: 0,
    };

    assert_eq!(time.year, 2025);
    assert_eq!(time.month, 10);
    assert_eq!(time.day, 3);
    assert_eq!(time.hour, 12);
    assert_eq!(time.minute, 30);
    assert_eq!(time.second, 45);
}

#[test]
fn test_time_validation_ranges() {
    // Valid time values
    let valid_time = Time {
        year: 2025,
        month: 12,
        day: 31,
        hour: 23,
        minute: 59,
        second: 59,
        pad1: 0,
        nanosecond: 999_999_999,
        time_zone: 0,
        daylight: 0,
        pad2: 0,
    };

    assert!(valid_time.month <= 12);
    assert!(valid_time.day <= 31);
    assert!(valid_time.hour < 24);
    assert!(valid_time.minute < 60);
    assert!(valid_time.second < 60);
    assert!(valid_time.nanosecond < 1_000_000_000);
}

#[test]
fn test_reset_type_copy_clone() {
    let reset1 = ResetType::EfiResetCold;
    let reset2 = reset1;

    assert_eq!(reset1, reset2);
}
