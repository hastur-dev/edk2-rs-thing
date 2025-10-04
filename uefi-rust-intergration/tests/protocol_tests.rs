// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Protocol Implementation Tests

#![cfg(test)]

use uefi_rust::protocols::*;
use uefi_rust::ffi::*;

// Mock protocol structures for testing
mod mocks {
    use super::*;

    pub fn create_mock_simple_text_output() -> SimpleTextOutputProtocol {
        SimpleTextOutputProtocol {
            reset: mock_reset,
            output_string: mock_output_string,
            test_string: mock_test_string,
            query_mode: mock_query_mode,
            set_mode: mock_set_mode,
            set_attribute: mock_set_attribute,
            clear_screen: mock_clear_screen,
            set_cursor_position: mock_set_cursor_position,
            enable_cursor: mock_enable_cursor,
            mode: core::ptr::null_mut(),
        }
    }

    unsafe extern "efiapi" fn mock_reset(
        _this: *mut SimpleTextOutputProtocol,
        _extended: Boolean,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_output_string(
        _this: *mut SimpleTextOutputProtocol,
        _string: *mut Char16,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_test_string(
        _this: *mut SimpleTextOutputProtocol,
        _string: *mut Char16,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_query_mode(
        _this: *mut SimpleTextOutputProtocol,
        _mode: Uintn,
        _columns: *mut Uintn,
        _rows: *mut Uintn,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_set_mode(
        _this: *mut SimpleTextOutputProtocol,
        _mode: Uintn,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_set_attribute(
        _this: *mut SimpleTextOutputProtocol,
        _attribute: Uintn,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_clear_screen(
        _this: *mut SimpleTextOutputProtocol,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_set_cursor_position(
        _this: *mut SimpleTextOutputProtocol,
        _column: Uintn,
        _row: Uintn,
    ) -> Status {
        EFI_SUCCESS
    }

    unsafe extern "efiapi" fn mock_enable_cursor(
        _this: *mut SimpleTextOutputProtocol,
        _visible: Boolean,
    ) -> Status {
        EFI_SUCCESS
    }
}

#[test]
fn test_simple_text_output_protocol() {
    let mut protocol = mocks::create_mock_simple_text_output();

    unsafe {
        let status = protocol.reset(false);
        assert_eq!(status, EFI_SUCCESS);

        let status = protocol.clear_screen();
        assert_eq!(status, EFI_SUCCESS);

        let status = protocol.set_cursor_position(0, 0);
        assert_eq!(status, EFI_SUCCESS);
    }
}

#[test]
fn test_graphics_output_blt_operation() {
    // Test BLT operation constants
    assert_eq!(EfiBltVideoFill, 0);
    assert_eq!(EfiBltVideoToBltBuffer, 1);
    assert_eq!(EfiBltBufferToVideo, 2);
    assert_eq!(EfiBltVideoToVideo, 3);
}

#[test]
fn test_graphics_output_pixel() {
    let pixel = GraphicsOutputBltPixel {
        blue: 255,
        green: 128,
        red: 64,
        reserved: 0,
    };

    assert_eq!(pixel.blue, 255);
    assert_eq!(pixel.green, 128);
    assert_eq!(pixel.red, 64);
    assert_eq!(pixel.reserved, 0);
}

#[test]
fn test_block_io_media() {
    let media = BlockIoMedia {
        media_id: 1,
        removable_media: 0,
        media_present: 1,
        logical_partition: 0,
        read_only: 0,
        write_caching: 0,
        block_size: 512,
        io_align: 1,
        last_block: 1000,
        lowest_aligned_lba: 0,
        logical_blocks_per_physical_block: 1,
        optimal_transfer_length_granularity: 1,
    };

    assert_eq!(media.block_size, 512);
    assert_eq!(media.last_block, 1000);
    assert_eq!(media.media_present, 1);
}

#[test]
fn test_file_open_modes() {
    assert_eq!(EFI_FILE_MODE_READ, 0x0000000000000001);
    assert_eq!(EFI_FILE_MODE_WRITE, 0x0000000000000002);
    assert_eq!(EFI_FILE_MODE_CREATE, 0x8000000000000000);
}

#[test]
fn test_file_attributes() {
    assert_eq!(EFI_FILE_READ_ONLY, 0x01);
    assert_eq!(EFI_FILE_HIDDEN, 0x02);
    assert_eq!(EFI_FILE_SYSTEM, 0x04);
    assert_eq!(EFI_FILE_DIRECTORY, 0x10);
    assert_eq!(EFI_FILE_ARCHIVE, 0x20);
}

#[test]
fn test_device_path_type() {
    assert_eq!(HARDWARE_DEVICE_PATH, 0x01);
    assert_eq!(ACPI_DEVICE_PATH, 0x02);
    assert_eq!(MESSAGING_DEVICE_PATH, 0x03);
    assert_eq!(MEDIA_DEVICE_PATH, 0x04);
    assert_eq!(END_DEVICE_PATH_TYPE, 0x7f);
}

#[test]
fn test_pci_bar_constants() {
    assert_eq!(PCI_BAR_IDX0, 0);
    assert_eq!(PCI_BAR_IDX1, 1);
    assert_eq!(PCI_BAR_IDX2, 2);
    assert_eq!(PCI_BAR_IDX3, 3);
    assert_eq!(PCI_BAR_IDX4, 4);
    assert_eq!(PCI_BAR_IDX5, 5);
}

#[test]
fn test_usb_data_direction() {
    assert_eq!(EfiUsbDataIn, 0);
    assert_eq!(EfiUsbDataOut, 1);
    assert_eq!(EfiUsbNoData, 2);
}

#[test]
fn test_network_receive_filters() {
    assert_eq!(EFI_SIMPLE_NETWORK_RECEIVE_UNICAST, 0x01);
    assert_eq!(EFI_SIMPLE_NETWORK_RECEIVE_MULTICAST, 0x02);
    assert_eq!(EFI_SIMPLE_NETWORK_RECEIVE_BROADCAST, 0x04);
    assert_eq!(EFI_SIMPLE_NETWORK_RECEIVE_PROMISCUOUS, 0x08);
}

#[test]
fn test_http_version() {
    assert_eq!(HttpVersion::Http10 as u32, 0);
    assert_eq!(HttpVersion::Http11 as u32, 1);
    assert_eq!(HttpVersion::Http20 as u32, 2);
}

#[test]
fn test_http_method() {
    assert_eq!(HttpMethod::HttpMethodGet as u32, 0);
    assert_eq!(HttpMethod::HttpMethodPost as u32, 1);
    assert_eq!(HttpMethod::HttpMethodPut as u32, 6);
    assert_eq!(HttpMethod::HttpMethodDelete as u32, 7);
}

#[test]
fn test_http_status_code() {
    assert_eq!(HttpStatusCode::Http200Ok as u32, 1);
    assert_eq!(HttpStatusCode::Http404NotFound as u32, 10);
    assert_eq!(HttpStatusCode::Http500InternalServerError as u32, 11);
}

#[test]
fn test_tcp_connection_state() {
    use tcp_udp::Tcp4ConnectionState;

    assert_eq!(Tcp4ConnectionState::Closed as u32, 0);
    assert_eq!(Tcp4ConnectionState::Listen as u32, 1);
    assert_eq!(Tcp4ConnectionState::Established as u32, 4);
}

#[test]
fn test_ipv4_address() {
    use tcp_udp::Ipv4Address;

    let addr = Ipv4Address {
        addr: [192, 168, 1, 1],
    };

    assert_eq!(addr.addr[0], 192);
    assert_eq!(addr.addr[1], 168);
    assert_eq!(addr.addr[2], 1);
    assert_eq!(addr.addr[3], 1);
}

#[test]
fn test_scsi_data_direction() {
    use storage::*;

    assert_eq!(SCSI_DATA_IN, 0);
    assert_eq!(SCSI_DATA_OUT, 1);
}

#[test]
fn test_scsi_commands() {
    use storage::scsi_commands::*;

    assert_eq!(SCSI_TEST_UNIT_READY, 0x00);
    assert_eq!(SCSI_INQUIRY, 0x12);
    assert_eq!(SCSI_READ_10, 0x28);
    assert_eq!(SCSI_WRITE_10, 0x2A);
}

#[test]
fn test_partition_type() {
    use storage::PartitionType;

    assert_eq!(PartitionType::Other as u32, 0);
    assert_eq!(PartitionType::Mbr as u32, 1);
    assert_eq!(PartitionType::Gpt as u32, 2);
}

#[test]
fn test_processor_flags() {
    use mp_services::*;

    assert_eq!(PROCESSOR_AS_BSP_BIT, 0x01);
    assert_eq!(PROCESSOR_ENABLED_BIT, 0x02);
    assert_eq!(PROCESSOR_HEALTH_STATUS_BIT, 0x04);
}

#[test]
fn test_processor_info_flags() {
    use mp_services::*;

    let info = ProcessorInformation {
        processor_id: 0,
        status_flag: PROCESSOR_AS_BSP_BIT | PROCESSOR_ENABLED_BIT,
        location: ProcessorLocation {
            package: 0,
            core: 0,
            thread: 0,
        },
        extended_information: ExtendedProcessorInformation {
            location: ProcessorLocation {
                package: 0,
                core: 0,
                thread: 0,
            },
        },
    };

    assert!(MpServicesProtocol::is_bsp(&info));
    assert!(MpServicesProtocol::is_enabled(&info));
    assert!(!MpServicesProtocol::is_healthy(&info));
}

#[test]
fn test_shell_status_codes() {
    use shell::*;

    assert_eq!(SHELL_SUCCESS, 0);
    assert_eq!(SHELL_INVALID_PARAMETER, 2);
    assert_eq!(SHELL_NOT_FOUND, 14);
    assert_eq!(SHELL_ACCESS_DENIED, 15);
}

#[test]
fn test_pxe_tftp_opcodes() {
    use pxe::*;

    assert_eq!(PXE_TFTP_OPCODE_RRQ, 1);
    assert_eq!(PXE_TFTP_OPCODE_WRQ, 2);
    assert_eq!(PXE_TFTP_OPCODE_DATA, 3);
    assert_eq!(PXE_TFTP_OPCODE_ACK, 4);
    assert_eq!(PXE_TFTP_OPCODE_ERROR, 5);
}

#[test]
fn test_hii_font_styles() {
    use hii::*;

    assert_eq!(HII_FONT_STYLE_NORMAL, 0x00000000);
    assert_eq!(HII_FONT_STYLE_BOLD, 0x00000001);
    assert_eq!(HII_FONT_STYLE_ITALIC, 0x00000002);
    assert_eq!(HII_FONT_STYLE_UNDERLINE, 0x00080000);
}

#[test]
fn test_hash_algorithm_guids() {
    use security::*;

    // Ensure GUIDs are properly defined and unique
    assert_ne!(HASH_ALGORITHM_SHA1_GUID, HASH_ALGORITHM_SHA256_GUID);
    assert_ne!(HASH_ALGORITHM_SHA256_GUID, HASH_ALGORITHM_SHA384_GUID);
    assert_ne!(HASH_ALGORITHM_SHA384_GUID, HASH_ALGORITHM_SHA512_GUID);
}

#[test]
fn test_cert_type_guids() {
    use security::*;

    assert_ne!(CERT_SHA256_GUID, CERT_RSA2048_GUID);
    assert_ne!(CERT_RSA2048_GUID, CERT_X509_GUID);
}

#[test]
fn test_rng_algorithm_guids() {
    use misc::*;

    assert_ne!(RNG_ALGORITHM_SP800_90_HASH_256_GUID, RNG_ALGORITHM_SP800_90_HMAC_256_GUID);
    assert_ne!(RNG_ALGORITHM_SP800_90_HMAC_256_GUID, RNG_ALGORITHM_SP800_90_CTR_256_GUID);
}

#[test]
fn test_protocol_guids_unique() {
    // Ensure all major protocol GUIDs are unique
    assert_ne!(SIMPLE_TEXT_INPUT_PROTOCOL_GUID, SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID);
    assert_ne!(GRAPHICS_OUTPUT_PROTOCOL_GUID, BLOCK_IO_PROTOCOL_GUID);
    assert_ne!(SIMPLE_FILE_SYSTEM_PROTOCOL_GUID, LOADED_IMAGE_PROTOCOL_GUID);
}

#[test]
fn test_service_binding_guids() {
    use tcp_udp::*;
    use ip::*;

    assert_ne!(TCP4_SERVICE_BINDING_PROTOCOL_GUID, TCP4_PROTOCOL_GUID);
    assert_ne!(UDP4_SERVICE_BINDING_PROTOCOL_GUID, UDP4_PROTOCOL_GUID);
    assert_ne!(IP4_SERVICE_BINDING_PROTOCOL_GUID, IP4_PROTOCOL_GUID);
}
