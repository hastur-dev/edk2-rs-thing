// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Integration Tests
//!
//! These tests verify that different components work together correctly.

#![cfg(test)]

mod mock_environment;

use mock_environment::*;
use uefi_rust::ffi::*;
use uefi_rust::guid::*;
use uefi_rust::protocols::*;
use uefi_rust::string::*;

#[test]
fn test_string_round_trip_conversion() {
    let original = "Hello, UEFI World! 你好";

    // Convert to UCS-2
    let ucs2 = utf8_to_ucs2(original);

    // Convert back to UTF-8
    let recovered = ucs2_to_utf8(&ucs2);

    assert_eq!(original, recovered);
}

#[test]
fn test_guid_parsing_and_formatting() {
    let guid_str = "12345678-1234-5678-9ABC-DEF012345678";

    let guid = parse_guid_string(guid_str).expect("Failed to parse GUID");

    assert_eq!(guid.data1, 0x12345678);
    assert_eq!(guid.data2, 0x1234);
    assert_eq!(guid.data3, 0x5678);

    // Format it back
    let formatted = format_guid(&guid);
    assert_eq!(formatted.to_lowercase(), guid_str.to_lowercase());
}

#[test]
fn test_multiple_string_conversions() {
    let test_strings = [
        "Simple ASCII",
        "UTF-8: 你好世界",
        "Emoji: 🎉🎊",
        "Mixed: Hello世界🌍",
        "",
        "A",
    ];

    for &test in &test_strings {
        let ucs2 = utf8_to_ucs2(test);
        let recovered = ucs2_to_utf8(&ucs2);
        assert_eq!(test, recovered, "Failed for: {}", test);
    }
}

#[test]
fn test_memory_allocation_and_deallocation() {
    let bs = MockBootServices::new();

    // Allocate multiple blocks
    let ptr1 = bs.allocate_pool(1024);
    let ptr2 = bs.allocate_pool(2048);
    let ptr3 = bs.allocate_pool(512);

    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!ptr3.is_null());

    // Verify they're different
    assert_ne!(ptr1, ptr2);
    assert_ne!(ptr2, ptr3);
    assert_ne!(ptr1, ptr3);

    // Free in different order
    bs.free_pool(ptr2);
    bs.free_pool(ptr1);
    bs.free_pool(ptr3);

    // All should be freed
    assert_eq!(bs.memory_allocations.lock().unwrap().len(), 0);
}

#[test]
fn test_tpl_nesting() {
    let bs = MockBootServices::new();

    let tpl0 = bs.get_current_tpl();
    assert_eq!(tpl0, 4); // TPL_APPLICATION

    let old1 = bs.raise_tpl(8); // TPL_CALLBACK
    assert_eq!(old1, 4);
    assert_eq!(bs.get_current_tpl(), 8);

    let old2 = bs.raise_tpl(16); // TPL_NOTIFY
    assert_eq!(old2, 8);
    assert_eq!(bs.get_current_tpl(), 16);

    bs.restore_tpl(old2);
    assert_eq!(bs.get_current_tpl(), 8);

    bs.restore_tpl(old1);
    assert_eq!(bs.get_current_tpl(), 4);
}

#[test]
fn test_variable_storage_and_retrieval() {
    let rs = MockRuntimeServices::new();

    let guid1 = Guid::new(1, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);
    let guid2 = Guid::new(2, 3, 4, [5, 6, 7, 8, 9, 10, 11, 12]);

    // Set multiple variables
    rs.set_variable("Var1", &guid1, vec![1, 2, 3]);
    rs.set_variable("Var2", &guid1, vec![4, 5, 6]);
    rs.set_variable("Var3", &guid2, vec![7, 8, 9]);

    // Retrieve and verify
    assert_eq!(rs.get_variable("Var1", &guid1), Some(vec![1, 2, 3]));
    assert_eq!(rs.get_variable("Var2", &guid1), Some(vec![4, 5, 6]));
    assert_eq!(rs.get_variable("Var3", &guid2), Some(vec![7, 8, 9]));

    // Wrong GUID should return None
    assert_eq!(rs.get_variable("Var1", &guid2), None);
    assert_eq!(rs.get_variable("Var3", &guid1), None);

    // Delete and verify
    rs.delete_variable("Var2", &guid1);
    assert_eq!(rs.get_variable("Var2", &guid1), None);

    // Others should still exist
    assert_eq!(rs.get_variable("Var1", &guid1), Some(vec![1, 2, 3]));
    assert_eq!(rs.get_variable("Var3", &guid2), Some(vec![7, 8, 9]));
}

#[test]
fn test_block_io_read_write_consistency() {
    let block_io = MockBlockIo::new(1000, 512);

    // Write pattern to multiple blocks
    for block in 0..10 {
        let pattern = vec![(block as u8); 512];
        block_io.write_blocks(block, &pattern).unwrap();
    }

    // Read back and verify
    for block in 0..10 {
        let mut buffer = vec![0u8; 512];
        block_io.read_blocks(block, &mut buffer).unwrap();

        assert_eq!(buffer, vec![(block as u8); 512]);
    }
}

#[test]
fn test_block_io_partial_operations() {
    let block_io = MockBlockIo::new(100, 512);

    // Write full block
    let write_data = vec![0xAA; 512];
    block_io.write_blocks(5, &write_data).unwrap();

    // Read back smaller portion (simulating partial block read)
    let mut read_data = vec![0; 256];
    block_io.read_blocks(5, &mut read_data).unwrap();

    assert_eq!(read_data, vec![0xAA; 256]);
}

#[test]
fn test_network_packet_queue() {
    let net = MockNetworkInterface::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

    // Inject multiple packets
    for i in 0..5 {
        net.inject_packet(vec![i; 64]);
    }

    // Receive in LIFO order (stack behavior)
    for i in (0..5).rev() {
        let packet = net.receive().unwrap();
        assert_eq!(packet, vec![i; 64]);
    }

    // Queue should be empty
    assert_eq!(net.receive(), None);
}

#[test]
fn test_network_transmit_tracking() {
    let net = MockNetworkInterface::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

    // Transmit several packets
    for i in 0..10 {
        let packet = vec![i; 100];
        net.transmit(packet);
    }

    // Verify all were tracked
    let transmitted = net.get_transmitted_packets();
    assert_eq!(transmitted.len(), 10);

    for (i, packet) in transmitted.iter().enumerate() {
        assert_eq!(packet, &vec![i as u8; 100]);
    }
}

#[test]
fn test_protocol_installation_and_lookup() {
    let mut env = MockUefiEnvironment::new();

    // Create and install multiple protocols
    let text_out = Box::into_raw(Box::new(MockSimpleTextOutput::new()));
    let block_io = Box::into_raw(Box::new(MockBlockIo::new(100, 512)));

    env.install_protocol(SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID, text_out);
    env.install_protocol(BLOCK_IO_PROTOCOL_GUID, block_io);

    // Locate protocols
    let found_text: Option<*mut MockSimpleTextOutput> =
        env.locate_protocol(&SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID);
    let found_block: Option<*mut MockBlockIo> = env.locate_protocol(&BLOCK_IO_PROTOCOL_GUID);

    assert!(found_text.is_some());
    assert!(found_block.is_some());

    // Try to locate non-existent protocol
    let guid = Guid::new(0xFFFFFFFF, 0xFFFF, 0xFFFF, [0xFF; 8]);
    let not_found: Option<*mut MockSimpleTextOutput> = env.locate_protocol(&guid);
    assert!(not_found.is_none());

    // Cleanup
    unsafe {
        let _ = Box::from_raw(text_out);
        let _ = Box::from_raw(block_io);
    }
}

#[test]
fn test_guid_comparison() {
    let guid1 = Guid::new(1, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);
    let guid2 = Guid::new(1, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);
    let guid3 = Guid::new(2, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);

    assert_eq!(guid1, guid2);
    assert_ne!(guid1, guid3);
    assert_ne!(guid2, guid3);
}

#[test]
fn test_status_code_values() {
    assert_eq!(EFI_SUCCESS, 0);
    assert_ne!(EFI_LOAD_ERROR, EFI_SUCCESS);
    assert_ne!(EFI_INVALID_PARAMETER, EFI_SUCCESS);
    assert_ne!(EFI_UNSUPPORTED, EFI_SUCCESS);
    assert_ne!(EFI_NOT_FOUND, EFI_SUCCESS);

    // Verify error codes have high bit set
    assert!(EFI_LOAD_ERROR & 0x8000_0000_0000_0000 != 0);
    assert!(EFI_INVALID_PARAMETER & 0x8000_0000_0000_0000 != 0);
}

#[test]
fn test_time_manipulation() {
    let rs = MockRuntimeServices::new();

    let mut time = rs.get_time();
    assert_eq!(time.year, 2025);

    // Advance by one hour
    time.hour += 1;
    rs.set_time(time);

    let new_time = rs.get_time();
    assert_eq!(new_time.hour, 13);
}

#[test]
fn test_event_creation_and_cleanup() {
    let bs = MockBootServices::new();

    // Create multiple events
    let event1 = bs.create_event(0x80000000, 4);
    let event2 = bs.create_event(0x00000200, 8);
    let event3 = bs.create_event(0x00000100, 16);

    assert_eq!(bs.events.lock().unwrap().len(), 3);

    // Close events
    bs.close_event(event1);
    bs.close_event(event2);

    // One event should remain
    assert_eq!(bs.events.lock().unwrap().len(), 1);

    bs.close_event(event3);
    assert_eq!(bs.events.lock().unwrap().len(), 0);
}

#[test]
fn test_comprehensive_environment_setup() {
    let mut env = MockUefiEnvironment::new();

    // Install all major protocol types
    let protocols = vec![
        (
            SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID,
            Box::into_raw(Box::new(MockSimpleTextOutput::new())) as *mut core::ffi::c_void,
        ),
        (
            BLOCK_IO_PROTOCOL_GUID,
            Box::into_raw(Box::new(MockBlockIo::new(100, 512))) as *mut core::ffi::c_void,
        ),
    ];

    for (guid, ptr) in &protocols {
        env.install_protocol(*guid, *ptr);
    }

    // Verify all are accessible
    assert!(env
        .locate_protocol::<MockSimpleTextOutput>(&SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID)
        .is_some());
    assert!(env
        .locate_protocol::<MockBlockIo>(&BLOCK_IO_PROTOCOL_GUID)
        .is_some());

    // Cleanup
    for (_, ptr) in protocols {
        unsafe {
            let _ = std::ptr::read(ptr);
        }
    }
}
