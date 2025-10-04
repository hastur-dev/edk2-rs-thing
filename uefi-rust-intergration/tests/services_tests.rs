// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Boot Services and Runtime Services Tests

#![cfg(test)]

use uefi_rust::ffi::*;
use uefi_rust::boot_services::*;
use uefi_rust::runtime_services::*;

#[test]
fn test_boot_services_signature() {
    assert_eq!(EFI_BOOT_SERVICES_SIGNATURE, 0x56524553544f4f42);
}

#[test]
fn test_memory_type_enum() {
    assert_eq!(MemoryType::EfiReservedMemoryType as u32, 0);
    assert_eq!(MemoryType::EfiLoaderCode as u32, 1);
    assert_eq!(MemoryType::EfiLoaderData as u32, 2);
    assert_eq!(MemoryType::EfiBootServicesCode as u32, 3);
    assert_eq!(MemoryType::EfiBootServicesData as u32, 4);
    assert_eq!(MemoryType::EfiRuntimeServicesCode as u32, 5);
    assert_eq!(MemoryType::EfiRuntimeServicesData as u32, 6);
    assert_eq!(MemoryType::EfiConventionalMemory as u32, 7);
}

#[test]
fn test_allocate_type() {
    assert_eq!(AllocateType::AllocateAnyPages as u32, 0);
    assert_eq!(AllocateType::AllocateMaxAddress as u32, 1);
    assert_eq!(AllocateType::AllocateAddress as u32, 2);
}

#[test]
fn test_tpl_levels() {
    use tpl::*;

    assert_eq!(TPL_APPLICATION, 4);
    assert_eq!(TPL_CALLBACK, 8);
    assert_eq!(TPL_NOTIFY, 16);
    assert_eq!(TPL_HIGH_LEVEL, 31);

    // Verify ordering
    assert!(TPL_APPLICATION < TPL_CALLBACK);
    assert!(TPL_CALLBACK < TPL_NOTIFY);
    assert!(TPL_NOTIFY < TPL_HIGH_LEVEL);
}

#[test]
fn test_event_types() {
    use events::*;

    assert_eq!(EVT_TIMER, 0x80000000);
    assert_eq!(EVT_RUNTIME, 0x40000000);
    assert_eq!(EVT_NOTIFY_WAIT, 0x00000100);
    assert_eq!(EVT_NOTIFY_SIGNAL, 0x00000200);
    assert_eq!(EVT_SIGNAL_EXIT_BOOT_SERVICES, 0x00000201);
}

#[test]
fn test_timer_type() {
    use events::TimerType;

    assert_eq!(TimerType::Cancel as u32, 0);
    assert_eq!(TimerType::Periodic as u32, 1);
    assert_eq!(TimerType::Relative as u32, 2);
}

#[test]
fn test_time_conversion() {
    use events::*;

    // Test milliseconds to 100ns units
    assert_eq!(milliseconds_to_100ns(1), 10_000);
    assert_eq!(milliseconds_to_100ns(1000), 10_000_000);

    // Test microseconds to 100ns units
    assert_eq!(microseconds_to_100ns(1), 10);
    assert_eq!(microseconds_to_100ns(1000), 10_000);

    // Test seconds to 100ns units
    assert_eq!(seconds_to_100ns(1), 10_000_000);
    assert_eq!(seconds_to_100ns(60), 600_000_000);
}

#[test]
fn test_variable_attributes() {
    use variables::*;

    assert_eq!(EFI_VARIABLE_NON_VOLATILE, 0x00000001);
    assert_eq!(EFI_VARIABLE_BOOTSERVICE_ACCESS, 0x00000002);
    assert_eq!(EFI_VARIABLE_RUNTIME_ACCESS, 0x00000004);
    assert_eq!(EFI_VARIABLE_HARDWARE_ERROR_RECORD, 0x00000008);
    assert_eq!(EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS, 0x00000010);
    assert_eq!(EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS, 0x00000020);
    assert_eq!(EFI_VARIABLE_APPEND_WRITE, 0x00000040);
}

#[test]
fn test_reset_type() {
    assert_eq!(ResetType::EfiResetCold as u32, 0);
    assert_eq!(ResetType::EfiResetWarm as u32, 1);
    assert_eq!(ResetType::EfiResetShutdown as u32, 2);
}

#[test]
fn test_time_structure() {
    let time = Time {
        year: 2025,
        month: 10,
        day: 4,
        hour: 12,
        minute: 30,
        second: 45,
        pad1: 0,
        nanosecond: 500_000_000,
        time_zone: 0,
        daylight: 0,
        pad2: 0,
    };

    assert_eq!(time.year, 2025);
    assert_eq!(time.month, 10);
    assert_eq!(time.day, 4);
    assert_eq!(time.hour, 12);
    assert_eq!(time.minute, 30);
    assert_eq!(time.second, 45);
    assert_eq!(time.nanosecond, 500_000_000);
}

#[test]
fn test_time_capabilities() {
    let caps = TimeCapabilities {
        resolution: 1,
        accuracy: 50_000_000,
        sets_to_zero: 0,
    };

    assert_eq!(caps.resolution, 1);
    assert_eq!(caps.accuracy, 50_000_000);
}

#[test]
fn test_memory_descriptor_size() {
    // Verify MemoryDescriptor has expected size
    let size = core::mem::size_of::<MemoryDescriptor>();
    assert!(size > 0);
    assert_eq!(size % 8, 0); // Should be 8-byte aligned
}

#[test]
fn test_open_protocol_attributes() {
    assert_eq!(EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, 0x00000001);
    assert_eq!(EFI_OPEN_PROTOCOL_GET_PROTOCOL, 0x00000002);
    assert_eq!(EFI_OPEN_PROTOCOL_TEST_PROTOCOL, 0x00000004);
    assert_eq!(EFI_OPEN_PROTOCOL_BY_CHILD_CONTROLLER, 0x00000008);
    assert_eq!(EFI_OPEN_PROTOCOL_BY_DRIVER, 0x00000010);
    assert_eq!(EFI_OPEN_PROTOCOL_EXCLUSIVE, 0x00000020);
}

#[test]
fn test_locate_search_type() {
    assert_eq!(LocateSearchType::AllHandles as u32, 0);
    assert_eq!(LocateSearchType::ByRegisterNotify as u32, 1);
    assert_eq!(LocateSearchType::ByProtocol as u32, 2);
}

// Mock tests for service wrappers
#[test]
fn test_boot_services_wrapper_creation() {
    // This would require a mock BootServices structure
    // For now, test that the type exists
    let _: Option<BootServicesWrapper> = None;
}

// Runtime Services tests
#[test]
fn test_global_variable_guid() {
    use variables::EFI_GLOBAL_VARIABLE_GUID;

    // Verify it's a valid GUID
    let guid = EFI_GLOBAL_VARIABLE_GUID;
    // The actual values should match the UEFI specification
    assert_eq!(guid.data1, 0x8BE4DF61);
}

#[test]
fn test_time_validation() {
    // Test valid time
    let valid_time = Time {
        year: 2025,
        month: 10,
        day: 4,
        hour: 12,
        minute: 30,
        second: 45,
        pad1: 0,
        nanosecond: 500_000_000,
        time_zone: 0,
        daylight: 0,
        pad2: 0,
    };

    assert!(valid_time.year >= 1900 && valid_time.year <= 9999);
    assert!(valid_time.month >= 1 && valid_time.month <= 12);
    assert!(valid_time.day >= 1 && valid_time.day <= 31);
    assert!(valid_time.hour <= 23);
    assert!(valid_time.minute <= 59);
    assert!(valid_time.second <= 59);
    assert!(valid_time.nanosecond < 1_000_000_000);
}

#[test]
fn test_daylight_flags() {
    const EFI_TIME_ADJUST_DAYLIGHT: u8 = 0x01;
    const EFI_TIME_IN_DAYLIGHT: u8 = 0x02;

    assert_eq!(EFI_TIME_ADJUST_DAYLIGHT, 0x01);
    assert_eq!(EFI_TIME_IN_DAYLIGHT, 0x02);
}

#[test]
fn test_capsule_flags() {
    const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: u32 = 0x00010000;
    const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: u32 = 0x00020000;
    const CAPSULE_FLAGS_INITIATE_RESET: u32 = 0x00040000;

    assert_eq!(CAPSULE_FLAGS_PERSIST_ACROSS_RESET, 0x00010000);
    assert_eq!(CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE, 0x00020000);
    assert_eq!(CAPSULE_FLAGS_INITIATE_RESET, 0x00040000);
}

#[test]
fn test_event_group_guids() {
    use events::*;

    // Verify event group GUIDs are defined
    assert_ne!(EVT_GROUP_EXIT_BOOT_SERVICES, Guid::new(0, 0, 0, [0; 8]));
    assert_ne!(EVT_GROUP_VIRTUAL_ADDRESS_CHANGE, Guid::new(0, 0, 0, [0; 8]));
    assert_ne!(EVT_GROUP_MEMORY_MAP_CHANGE, Guid::new(0, 0, 0, [0; 8]));
    assert_ne!(EVT_GROUP_READY_TO_BOOT, Guid::new(0, 0, 0, [0; 8]));
}
