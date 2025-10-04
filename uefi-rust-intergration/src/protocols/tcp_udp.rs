// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI TCP/UDP Protocol - Network Transport Layer

use crate::ffi::*;
use crate::runtime_services::Time;

/// EFI_TCP4_SERVICE_BINDING_PROTOCOL_GUID
pub const TCP4_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x00720665,
    0x67EB,
    0x4a99,
    [0xBA, 0xF7, 0xD3, 0xC3, 0x3A, 0x1C, 0x7C, 0xC9],
);

/// EFI_TCP4_PROTOCOL_GUID
pub const TCP4_PROTOCOL_GUID: Guid = Guid::new(
    0x65530BC7,
    0xA359,
    0x410f,
    [0xB0, 0x10, 0x5A, 0xAD, 0xC7, 0xEC, 0x2B, 0x62],
);

/// EFI_TCP6_SERVICE_BINDING_PROTOCOL_GUID
pub const TCP6_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0xec20eb79,
    0x6c1a,
    0x4664,
    [0x9a, 0x0d, 0xd2, 0xe4, 0xcc, 0x16, 0xd6, 0x64],
);

/// EFI_TCP6_PROTOCOL_GUID
pub const TCP6_PROTOCOL_GUID: Guid = Guid::new(
    0x46e44855,
    0xbd60,
    0x4ab7,
    [0xab, 0x0d, 0xa6, 0x79, 0xb9, 0x44, 0x7d, 0x77],
);

/// EFI_UDP4_SERVICE_BINDING_PROTOCOL_GUID
pub const UDP4_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x3ad9df29,
    0x4501,
    0x478d,
    [0xb1, 0xf8, 0x7f, 0x7f, 0xe7, 0x0e, 0x50, 0xf3],
);

/// EFI_UDP4_PROTOCOL_GUID
pub const UDP4_PROTOCOL_GUID: Guid = Guid::new(
    0x3ad9df29,
    0x4501,
    0x478d,
    [0xb1, 0xf8, 0x7f, 0x7f, 0xe7, 0x0e, 0x50, 0xf3],
);

/// EFI_UDP6_SERVICE_BINDING_PROTOCOL_GUID
pub const UDP6_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x66ed4721,
    0x3c98,
    0x4d3e,
    [0x81, 0xe3, 0xd0, 0x3d, 0xd3, 0x9a, 0x72, 0x54],
);

/// EFI_UDP6_PROTOCOL_GUID
pub const UDP6_PROTOCOL_GUID: Guid = Guid::new(
    0x4f948815,
    0xb4b9,
    0x43cb,
    [0x8a, 0x33, 0x90, 0xe0, 0x60, 0xb3, 0x49, 0x55],
);

/// TCP Connection State
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tcp4ConnectionState {
    Closed = 0,
    Listen = 1,
    SynSent = 2,
    SynReceived = 3,
    Established = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    Closing = 7,
    TimeWait = 8,
    CloseWait = 9,
    LastAck = 10,
}

/// IPv4 Address
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ipv4Address {
    pub addr: [Uint8; 4],
}

/// IPv6 Address
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ipv6Address {
    pub addr: [Uint8; 16],
}

/// TCP4 Access Point
#[repr(C)]
pub struct Tcp4AccessPoint {
    pub use_default_address: Boolean,
    pub station_address: Ipv4Address,
    pub subnet_mask: Ipv4Address,
    pub station_port: Uint16,
    pub remote_address: Ipv4Address,
    pub remote_port: Uint16,
    pub active_flag: Boolean,
}

/// TCP6 Access Point
#[repr(C)]
pub struct Tcp6AccessPoint {
    pub station_address: Ipv6Address,
    pub station_port: Uint16,
    pub remote_address: Ipv6Address,
    pub remote_port: Uint16,
    pub active_flag: Boolean,
}

/// TCP4 Option
#[repr(C)]
pub struct Tcp4Option {
    pub receive_buffer_size: Uint32,
    pub send_buffer_size: Uint32,
    pub max_syn_back_log: Uint32,
    pub connection_timeout: Uint32,
    pub data_retries: Uint32,
    pub fin_timeout: Uint32,
    pub time_wait_timeout: Uint32,
    pub keep_alive_probes: Uint32,
    pub keep_alive_time: Uint32,
    pub keep_alive_interval: Uint32,
    pub enable_nagle: Boolean,
    pub enable_time_stamp: Boolean,
    pub enable_window_scaling: Boolean,
    pub enable_selective_ack: Boolean,
    pub enable_path_mtu_discovery: Boolean,
}

/// TCP6 Option
#[repr(C)]
pub struct Tcp6Option {
    pub receive_buffer_size: Uint32,
    pub send_buffer_size: Uint32,
    pub max_syn_back_log: Uint32,
    pub connection_timeout: Uint32,
    pub data_retries: Uint32,
    pub fin_timeout: Uint32,
    pub time_wait_timeout: Uint32,
    pub keep_alive_probes: Uint32,
    pub keep_alive_time: Uint32,
    pub keep_alive_interval: Uint32,
    pub enable_nagle: Boolean,
    pub enable_time_stamp: Boolean,
    pub enable_window_scaling: Boolean,
    pub enable_selective_ack: Boolean,
    pub enable_path_mtu_discovery: Boolean,
}

/// TCP4 Configuration Data
#[repr(C)]
pub struct Tcp4ConfigData {
    pub type_of_service: Uint8,
    pub time_to_live: Uint8,
    pub access_point: Tcp4AccessPoint,
    pub control_option: *mut Tcp4Option,
}

/// TCP6 Configuration Data
#[repr(C)]
pub struct Tcp6ConfigData {
    pub traffic_class: Uint8,
    pub hop_limit: Uint8,
    pub access_point: Tcp6AccessPoint,
    pub control_option: *mut Tcp6Option,
}

/// TCP4 Fragment Data
#[repr(C)]
pub struct Tcp4FragmentData {
    pub fragment_length: Uint32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

/// TCP6 Fragment Data
#[repr(C)]
pub struct Tcp6FragmentData {
    pub fragment_length: Uint32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

/// TCP4 Receive Data
#[repr(C)]
pub struct Tcp4ReceiveData {
    pub urgent_flag: Boolean,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Tcp4FragmentData; 1],
}

/// TCP6 Receive Data
#[repr(C)]
pub struct Tcp6ReceiveData {
    pub urgent_flag: Boolean,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Tcp6FragmentData; 1],
}

/// TCP4 Transmit Data
#[repr(C)]
pub struct Tcp4TransmitData {
    pub push: Boolean,
    pub urgent: Boolean,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Tcp4FragmentData; 1],
}

/// TCP6 Transmit Data
#[repr(C)]
pub struct Tcp6TransmitData {
    pub push: Boolean,
    pub urgent: Boolean,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Tcp6FragmentData; 1],
}

/// TCP4 I/O Token
#[repr(C)]
pub struct Tcp4IoToken {
    pub completion_token: Tcp4CompletionToken,
    pub packet: Tcp4IoTokenPacket,
}

/// TCP6 I/O Token
#[repr(C)]
pub struct Tcp6IoToken {
    pub completion_token: Tcp6CompletionToken,
    pub packet: Tcp6IoTokenPacket,
}

/// TCP4 Completion Token
#[repr(C)]
pub struct Tcp4CompletionToken {
    pub event: Event,
    pub status: Status,
}

/// TCP6 Completion Token
#[repr(C)]
pub struct Tcp6CompletionToken {
    pub event: Event,
    pub status: Status,
}

/// TCP4 I/O Token Packet Union
#[repr(C)]
pub union Tcp4IoTokenPacket {
    pub rx_data: *mut Tcp4ReceiveData,
    pub tx_data: *mut Tcp4TransmitData,
}

/// TCP6 I/O Token Packet Union
#[repr(C)]
pub union Tcp6IoTokenPacket {
    pub rx_data: *mut Tcp6ReceiveData,
    pub tx_data: *mut Tcp6TransmitData,
}

/// EFI_TCP4_PROTOCOL
#[repr(C)]
pub struct Tcp4Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        tcp4_state: *mut Tcp4ConnectionState,
        tcp4_config_data: *mut Tcp4ConfigData,
        ip4_mode_data: *mut core::ffi::c_void,
        mnp_config_data: *mut core::ffi::c_void,
        snp_mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        tcp_config_data: *const Tcp4ConfigData,
    ) -> Status,
    pub routes: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        delete_route: Boolean,
        subnet_address: *const Ipv4Address,
        subnet_mask: *const Ipv4Address,
        gateway_address: *const Ipv4Address,
    ) -> Status,
    pub connect: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        connection_token: *mut Tcp4CompletionToken,
    ) -> Status,
    pub accept: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        listen_token: *mut Tcp4CompletionToken,
    ) -> Status,
    pub transmit: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        token: *mut Tcp4IoToken,
    ) -> Status,
    pub receive: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        token: *mut Tcp4IoToken,
    ) -> Status,
    pub close: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        close_token: *mut Tcp4CompletionToken,
    ) -> Status,
    pub cancel: unsafe extern "efiapi" fn(
        this: *mut Tcp4Protocol,
        token: *mut Tcp4CompletionToken,
    ) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Tcp4Protocol) -> Status,
}

/// EFI_TCP6_PROTOCOL
#[repr(C)]
pub struct Tcp6Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        tcp6_state: *mut Tcp4ConnectionState,
        tcp6_config_data: *mut Tcp6ConfigData,
        ip6_mode_data: *mut core::ffi::c_void,
        mnp_config_data: *mut core::ffi::c_void,
        snp_mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        tcp_config_data: *const Tcp6ConfigData,
    ) -> Status,
    pub connect: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        connection_token: *mut Tcp6CompletionToken,
    ) -> Status,
    pub accept: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        listen_token: *mut Tcp6CompletionToken,
    ) -> Status,
    pub transmit: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        token: *mut Tcp6IoToken,
    ) -> Status,
    pub receive: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        token: *mut Tcp6IoToken,
    ) -> Status,
    pub close: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        close_token: *mut Tcp6CompletionToken,
    ) -> Status,
    pub cancel: unsafe extern "efiapi" fn(
        this: *mut Tcp6Protocol,
        token: *mut Tcp6CompletionToken,
    ) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Tcp6Protocol) -> Status,
}

/// UDP4 Configuration Data
#[repr(C)]
pub struct Udp4ConfigData {
    pub accept_broadcast: Boolean,
    pub accept_promiscuous: Boolean,
    pub accept_any_port: Boolean,
    pub allow_duplicate_port: Boolean,
    pub type_of_service: Uint8,
    pub time_to_live: Uint8,
    pub do_not_fragment: Boolean,
    pub receive_timeout: Uint32,
    pub transmit_timeout: Uint32,
    pub use_default_address: Boolean,
    pub station_address: Ipv4Address,
    pub subnet_mask: Ipv4Address,
    pub station_port: Uint16,
    pub remote_address: Ipv4Address,
    pub remote_port: Uint16,
}

/// UDP6 Configuration Data
#[repr(C)]
pub struct Udp6ConfigData {
    pub accept_promiscuous: Boolean,
    pub accept_any_port: Boolean,
    pub allow_duplicate_port: Boolean,
    pub traffic_class: Uint8,
    pub hop_limit: Uint8,
    pub receive_timeout: Uint32,
    pub transmit_timeout: Uint32,
    pub station_address: Ipv6Address,
    pub station_port: Uint16,
    pub remote_address: Ipv6Address,
    pub remote_port: Uint16,
}

/// UDP4 Fragment Data
#[repr(C)]
pub struct Udp4FragmentData {
    pub fragment_length: Uint32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

/// UDP6 Fragment Data
#[repr(C)]
pub struct Udp6FragmentData {
    pub fragment_length: Uint32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

/// UDP4 Receive Data
#[repr(C)]
pub struct Udp4ReceiveData {
    pub time_stamp: Time,
    pub recycle_signal: Event,
    pub udp4_session: Udp4SessionData,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Udp4FragmentData; 1],
}

/// UDP6 Receive Data
#[repr(C)]
pub struct Udp6ReceiveData {
    pub time_stamp: Time,
    pub recycle_signal: Event,
    pub udp6_session: Udp6SessionData,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Udp6FragmentData; 1],
}

/// UDP4 Transmit Data
#[repr(C)]
pub struct Udp4TransmitData {
    pub udp4_session_data: *mut Udp4SessionData,
    pub gateway_address: *mut Ipv4Address,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Udp4FragmentData; 1],
}

/// UDP6 Transmit Data
#[repr(C)]
pub struct Udp6TransmitData {
    pub udp6_session_data: *mut Udp6SessionData,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Udp6FragmentData; 1],
}

/// UDP4 Session Data
#[repr(C)]
pub struct Udp4SessionData {
    pub source_address: Ipv4Address,
    pub source_port: Uint16,
    pub destination_address: Ipv4Address,
    pub destination_port: Uint16,
}

/// UDP6 Session Data
#[repr(C)]
pub struct Udp6SessionData {
    pub source_address: Ipv6Address,
    pub source_port: Uint16,
    pub destination_address: Ipv6Address,
    pub destination_port: Uint16,
}

/// UDP4 Completion Token
#[repr(C)]
pub struct Udp4CompletionToken {
    pub event: Event,
    pub status: Status,
    pub packet: Udp4CompletionTokenPacket,
}

/// UDP6 Completion Token
#[repr(C)]
pub struct Udp6CompletionToken {
    pub event: Event,
    pub status: Status,
    pub packet: Udp6CompletionTokenPacket,
}

/// UDP4 Completion Token Packet Union
#[repr(C)]
pub union Udp4CompletionTokenPacket {
    pub rx_data: *mut Udp4ReceiveData,
    pub tx_data: *mut Udp4TransmitData,
}

/// UDP6 Completion Token Packet Union
#[repr(C)]
pub union Udp6CompletionTokenPacket {
    pub rx_data: *mut Udp6ReceiveData,
    pub tx_data: *mut Udp6TransmitData,
}

/// EFI_UDP4_PROTOCOL
#[repr(C)]
pub struct Udp4Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        udp4_config_data: *mut Udp4ConfigData,
        ip4_mode_data: *mut core::ffi::c_void,
        mnp_config_data: *mut core::ffi::c_void,
        snp_mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        udp_config_data: *const Udp4ConfigData,
    ) -> Status,
    pub groups: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        join_flag: Boolean,
        multicast_address: *const Ipv4Address,
    ) -> Status,
    pub routes: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        delete_route: Boolean,
        subnet_address: *const Ipv4Address,
        subnet_mask: *const Ipv4Address,
        gateway_address: *const Ipv4Address,
    ) -> Status,
    pub transmit: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        token: *mut Udp4CompletionToken,
    ) -> Status,
    pub receive: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        token: *mut Udp4CompletionToken,
    ) -> Status,
    pub cancel: unsafe extern "efiapi" fn(
        this: *mut Udp4Protocol,
        token: *mut Udp4CompletionToken,
    ) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Udp4Protocol) -> Status,
}

/// EFI_UDP6_PROTOCOL
#[repr(C)]
pub struct Udp6Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Udp6Protocol,
        udp6_config_data: *mut Udp6ConfigData,
        ip6_mode_data: *mut core::ffi::c_void,
        mnp_config_data: *mut core::ffi::c_void,
        snp_mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Udp6Protocol,
        udp_config_data: *const Udp6ConfigData,
    ) -> Status,
    pub groups: unsafe extern "efiapi" fn(
        this: *mut Udp6Protocol,
        join_flag: Boolean,
        multicast_address: *const Ipv6Address,
    ) -> Status,
    pub transmit: unsafe extern "efiapi" fn(
        this: *mut Udp6Protocol,
        token: *mut Udp6CompletionToken,
    ) -> Status,
    pub receive: unsafe extern "efiapi" fn(
        this: *mut Udp6Protocol,
        token: *mut Udp6CompletionToken,
    ) -> Status,
    pub cancel: unsafe extern "efiapi" fn(
        this: *mut Udp6Protocol,
        token: *mut Udp6CompletionToken,
    ) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Udp6Protocol) -> Status,
}

impl Tcp4Protocol {
    /// Configure TCP4 connection
    pub unsafe fn configure(&mut self, config: Option<&Tcp4ConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Connect to remote endpoint
    pub unsafe fn connect(&mut self, token: &mut Tcp4CompletionToken) -> Status {
        (self.connect)(self, token as *mut _)
    }

    /// Transmit data
    pub unsafe fn transmit(&mut self, token: &mut Tcp4IoToken) -> Status {
        (self.transmit)(self, token as *mut _)
    }

    /// Receive data
    pub unsafe fn receive(&mut self, token: &mut Tcp4IoToken) -> Status {
        (self.receive)(self, token as *mut _)
    }

    /// Close connection
    pub unsafe fn close(&mut self, token: &mut Tcp4CompletionToken) -> Status {
        (self.close)(self, token as *mut _)
    }
}

impl Tcp6Protocol {
    /// Configure TCP6 connection
    pub unsafe fn configure(&mut self, config: Option<&Tcp6ConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Connect to remote endpoint
    pub unsafe fn connect(&mut self, token: &mut Tcp6CompletionToken) -> Status {
        (self.connect)(self, token as *mut _)
    }

    /// Transmit data
    pub unsafe fn transmit(&mut self, token: &mut Tcp6IoToken) -> Status {
        (self.transmit)(self, token as *mut _)
    }

    /// Receive data
    pub unsafe fn receive(&mut self, token: &mut Tcp6IoToken) -> Status {
        (self.receive)(self, token as *mut _)
    }

    /// Close connection
    pub unsafe fn close(&mut self, token: &mut Tcp6CompletionToken) -> Status {
        (self.close)(self, token as *mut _)
    }
}

impl Udp4Protocol {
    /// Configure UDP4
    pub unsafe fn configure(&mut self, config: Option<&Udp4ConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Transmit datagram
    pub unsafe fn transmit(&mut self, token: &mut Udp4CompletionToken) -> Status {
        (self.transmit)(self, token as *mut _)
    }

    /// Receive datagram
    pub unsafe fn receive(&mut self, token: &mut Udp4CompletionToken) -> Status {
        (self.receive)(self, token as *mut _)
    }
}

impl Udp6Protocol {
    /// Configure UDP6
    pub unsafe fn configure(&mut self, config: Option<&Udp6ConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Transmit datagram
    pub unsafe fn transmit(&mut self, token: &mut Udp6CompletionToken) -> Status {
        (self.transmit)(self, token as *mut _)
    }

    /// Receive datagram
    pub unsafe fn receive(&mut self, token: &mut Udp6CompletionToken) -> Status {
        (self.receive)(self, token as *mut _)
    }
}
