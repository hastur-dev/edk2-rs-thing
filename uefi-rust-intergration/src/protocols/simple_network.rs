// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Simple Network Protocol

use crate::ffi::*;

/// EFI_SIMPLE_NETWORK_PROTOCOL_GUID
pub const SIMPLE_NETWORK_PROTOCOL_GUID: Guid = Guid::new(
    0xA19832B9,
    0xAC25,
    0x11D3,
    [0x9A, 0x2D, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

pub const MAX_MCAST_FILTER_CNT: usize = 16;

/// EFI_SIMPLE_NETWORK_STATE
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SimpleNetworkState {
    EfiSimpleNetworkStopped = 0,
    EfiSimpleNetworkStarted = 1,
    EfiSimpleNetworkInitialized = 2,
    EfiSimpleNetworkMaxState = 3,
}

/// EFI_MAC_ADDRESS
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MacAddress {
    pub addr: [Uint8; 32],
}

/// EFI_NETWORK_STATISTICS
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetworkStatistics {
    pub rx_total_frames: Uint64,
    pub rx_good_frames: Uint64,
    pub rx_undersize_frames: Uint64,
    pub rx_oversize_frames: Uint64,
    pub rx_dropped_frames: Uint64,
    pub rx_unicast_frames: Uint64,
    pub rx_broadcast_frames: Uint64,
    pub rx_multicast_frames: Uint64,
    pub rx_crc_error_frames: Uint64,
    pub rx_total_bytes: Uint64,
    pub tx_total_frames: Uint64,
    pub tx_good_frames: Uint64,
    pub tx_undersize_frames: Uint64,
    pub tx_oversize_frames: Uint64,
    pub tx_dropped_frames: Uint64,
    pub tx_unicast_frames: Uint64,
    pub tx_broadcast_frames: Uint64,
    pub tx_multicast_frames: Uint64,
    pub tx_crc_error_frames: Uint64,
    pub tx_total_bytes: Uint64,
    pub collisions: Uint64,
    pub unsupported_protocol: Uint64,
}

/// EFI_SIMPLE_NETWORK_MODE
#[repr(C)]
pub struct SimpleNetworkMode {
    pub state: Uint32,
    pub hw_address_size: Uint32,
    pub media_header_size: Uint32,
    pub max_packet_size: Uint32,
    pub nvram_size: Uint32,
    pub nvram_access_size: Uint32,
    pub receive_filter_mask: Uint32,
    pub receive_filter_setting: Uint32,
    pub max_mcast_filter_count: Uint32,
    pub mcast_filter_count: Uint32,
    pub mcast_filter: [MacAddress; MAX_MCAST_FILTER_CNT],
    pub current_address: MacAddress,
    pub broadcast_address: MacAddress,
    pub permanent_address: MacAddress,
    pub if_type: Uint8,
    pub mac_address_changeable: Boolean,
    pub multiple_tx_supported: Boolean,
    pub media_present_supported: Boolean,
    pub media_present: Boolean,
}

/// EFI_SIMPLE_NETWORK_PROTOCOL
#[repr(C)]
pub struct SimpleNetworkProtocol {
    pub revision: Uint64,
    pub start: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
    ) -> Status,
    pub stop: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
    ) -> Status,
    pub initialize: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        extra_rx_buffer_size: Uintn,
        extra_tx_buffer_size: Uintn,
    ) -> Status,
    pub reset: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        extended_verification: Boolean,
    ) -> Status,
    pub shutdown: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
    ) -> Status,
    pub receive_filters: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        enable: Uint32,
        disable: Uint32,
        reset_mcast_filter: Boolean,
        mcast_filter_cnt: Uintn,
        mcast_filter: *const MacAddress,
    ) -> Status,
    pub station_address: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        reset: Boolean,
        new_mac: *const MacAddress,
    ) -> Status,
    pub statistics: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        reset: Boolean,
        statistics_size: *mut Uintn,
        statistics_table: *mut NetworkStatistics,
    ) -> Status,
    pub mcast_ip_to_mac: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        ipv6: Boolean,
        ip: *const core::ffi::c_void,
        mac: *mut MacAddress,
    ) -> Status,
    pub nvdata: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        read_write: Boolean,
        offset: Uintn,
        buffer_size: Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub get_status: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        interrupt_status: *mut Uint32,
        tx_buf: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub transmit: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        header_size: Uintn,
        buffer_size: Uintn,
        buffer: *const core::ffi::c_void,
        src_addr: *const MacAddress,
        dest_addr: *const MacAddress,
        protocol: *const Uint16,
    ) -> Status,
    pub receive: unsafe extern "efiapi" fn(
        this: *mut SimpleNetworkProtocol,
        header_size: *mut Uintn,
        buffer_size: *mut Uintn,
        buffer: *mut core::ffi::c_void,
        src_addr: *mut MacAddress,
        dest_addr: *mut MacAddress,
        protocol: *mut Uint16,
    ) -> Status,
    pub wait_for_packet: Event,
    pub mode: *mut SimpleNetworkMode,
}

// Receive filter settings
pub const EFI_SIMPLE_NETWORK_RECEIVE_UNICAST: u32 = 0x01;
pub const EFI_SIMPLE_NETWORK_RECEIVE_MULTICAST: u32 = 0x02;
pub const EFI_SIMPLE_NETWORK_RECEIVE_BROADCAST: u32 = 0x04;
pub const EFI_SIMPLE_NETWORK_RECEIVE_PROMISCUOUS: u32 = 0x08;
pub const EFI_SIMPLE_NETWORK_RECEIVE_PROMISCUOUS_MULTICAST: u32 = 0x10;

// Interrupt status bits
pub const EFI_SIMPLE_NETWORK_RECEIVE_INTERRUPT: u32 = 0x01;
pub const EFI_SIMPLE_NETWORK_TRANSMIT_INTERRUPT: u32 = 0x02;
pub const EFI_SIMPLE_NETWORK_COMMAND_INTERRUPT: u32 = 0x04;
pub const EFI_SIMPLE_NETWORK_SOFTWARE_INTERRUPT: u32 = 0x08;

impl SimpleNetworkProtocol {
    /// Start the network interface
    pub unsafe fn start(&mut self) -> Status {
        (self.start)(self)
    }

    /// Stop the network interface
    pub unsafe fn stop(&mut self) -> Status {
        (self.stop)(self)
    }

    /// Initialize the network interface
    pub unsafe fn initialize(&mut self, extra_rx_size: usize, extra_tx_size: usize) -> Status {
        (self.initialize)(self, extra_rx_size, extra_tx_size)
    }

    /// Reset the network interface
    pub unsafe fn reset(&mut self, extended_verification: bool) -> Status {
        (self.reset)(self, extended_verification as Boolean)
    }

    /// Shutdown the network interface
    pub unsafe fn shutdown(&mut self) -> Status {
        (self.shutdown)(self)
    }

    /// Transmit a packet
    pub unsafe fn transmit(
        &mut self,
        buffer: &[u8],
        src_addr: Option<&MacAddress>,
        dest_addr: Option<&MacAddress>,
        protocol: Option<u16>,
    ) -> Status {
        let src_ptr = src_addr.map_or(core::ptr::null(), |a| a as *const _);
        let dest_ptr = dest_addr.map_or(core::ptr::null(), |a| a as *const _);
        let proto_ptr = protocol.as_ref().map_or(core::ptr::null(), |p| p as *const _);

        (self.transmit)(
            self,
            0,
            buffer.len(),
            buffer.as_ptr() as *const core::ffi::c_void,
            src_ptr,
            dest_ptr,
            proto_ptr,
        )
    }

    /// Receive a packet
    pub unsafe fn receive(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<(usize, MacAddress, MacAddress, u16), Status> {
        let mut header_size = 0;
        let mut buffer_size = buffer.len();
        let mut src_addr = core::mem::zeroed();
        let mut dest_addr = core::mem::zeroed();
        let mut protocol = 0u16;

        let status = (self.receive)(
            self,
            &mut header_size,
            &mut buffer_size,
            buffer.as_mut_ptr() as *mut core::ffi::c_void,
            &mut src_addr,
            &mut dest_addr,
            &mut protocol,
        );

        if status == EFI_SUCCESS {
            Ok((buffer_size, src_addr, dest_addr, protocol))
        } else {
            Err(status)
        }
    }

    /// Get network statistics
    pub unsafe fn get_statistics(&mut self, reset: bool) -> Result<NetworkStatistics, Status> {
        let mut size = core::mem::size_of::<NetworkStatistics>();
        let mut stats = core::mem::zeroed();

        let status = (self.statistics)(self, reset as Boolean, &mut size, &mut stats);

        if status == EFI_SUCCESS {
            Ok(stats)
        } else {
            Err(status)
        }
    }

    /// Get current mode
    pub unsafe fn get_mode(&self) -> Option<&SimpleNetworkMode> {
        if self.mode.is_null() {
            None
        } else {
            Some(&*self.mode)
        }
    }
}
