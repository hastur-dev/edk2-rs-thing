// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI IP4/IP6 and Related Network Protocols

use crate::ffi::*;
use crate::protocols::tcp_udp::{Ipv4Address, Ipv6Address};
use crate::runtime_services::Time;

/// EFI_IP4_SERVICE_BINDING_PROTOCOL_GUID
pub const IP4_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0xc51711e7,
    0xb4bf,
    0x404a,
    [0xbf, 0xb8, 0x0a, 0x04, 0x8e, 0xf1, 0xff, 0xe4],
);

/// EFI_IP4_PROTOCOL_GUID
pub const IP4_PROTOCOL_GUID: Guid = Guid::new(
    0x41d94cd2,
    0x35b6,
    0x455a,
    [0x82, 0x58, 0xd4, 0xe5, 0x13, 0x34, 0xaa, 0xdd],
);

/// EFI_IP6_SERVICE_BINDING_PROTOCOL_GUID
pub const IP6_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0xec835dd3,
    0xfe0f,
    0x617b,
    [0xa6, 0x21, 0xb3, 0x50, 0xc3, 0xe1, 0x33, 0x88],
);

/// EFI_IP6_PROTOCOL_GUID
pub const IP6_PROTOCOL_GUID: Guid = Guid::new(
    0x2c8759d5,
    0x5c2d,
    0x66ef,
    [0x92, 0x5f, 0xb6, 0x6c, 0x10, 0x19, 0x57, 0xe2],
);

/// EFI_ARP_SERVICE_BINDING_PROTOCOL_GUID
pub const ARP_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0xf44c00ee,
    0x1f2c,
    0x4a00,
    [0xaa, 0x09, 0x1c, 0x9f, 0x3e, 0x08, 0x00, 0xa3],
);

/// EFI_ARP_PROTOCOL_GUID
pub const ARP_PROTOCOL_GUID: Guid = Guid::new(
    0xf4b427bb,
    0xba21,
    0x4f16,
    [0xbc, 0x4e, 0x43, 0xe4, 0x16, 0xab, 0x61, 0x9c],
);

/// EFI_DHCP4_SERVICE_BINDING_PROTOCOL_GUID
pub const DHCP4_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x9d9a39d8,
    0xbd42,
    0x4a73,
    [0xa4, 0xd5, 0x8e, 0xe9, 0x4b, 0xe1, 0x13, 0x80],
);

/// EFI_DHCP4_PROTOCOL_GUID
pub const DHCP4_PROTOCOL_GUID: Guid = Guid::new(
    0x8a219718,
    0x4ef5,
    0x4761,
    [0x91, 0xc8, 0xc0, 0xf0, 0x4b, 0xda, 0x9e, 0x56],
);

/// EFI_DHCP6_SERVICE_BINDING_PROTOCOL_GUID
pub const DHCP6_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x9fb9a8a1,
    0x2f4a,
    0x43a6,
    [0x88, 0x9c, 0xd0, 0xf7, 0xb6, 0xc4, 0x7a, 0xd5],
);

/// EFI_DHCP6_PROTOCOL_GUID
pub const DHCP6_PROTOCOL_GUID: Guid = Guid::new(
    0x87c8bad7,
    0x595,
    0x4053,
    [0x82, 0x97, 0xde, 0xde, 0x39, 0x5f, 0x5d, 0x5b],
);

/// EFI_DNS4_SERVICE_BINDING_PROTOCOL_GUID
pub const DNS4_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0xb625b186,
    0xe063,
    0x44f7,
    [0x89, 0x05, 0x6a, 0x74, 0xdc, 0x6f, 0x52, 0xb4],
);

/// EFI_DNS4_PROTOCOL_GUID
pub const DNS4_PROTOCOL_GUID: Guid = Guid::new(
    0xae3d28cc,
    0xe05b,
    0x4fa1,
    [0xa0, 0x11, 0x7e, 0xb5, 0x5a, 0x3f, 0x14, 0x01],
);

/// EFI_DNS6_SERVICE_BINDING_PROTOCOL_GUID
pub const DNS6_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x7f1647c8,
    0xb76e,
    0x44b2,
    [0xa5, 0x65, 0xf7, 0x0f, 0xf1, 0x9c, 0xd1, 0x9e],
);

/// EFI_DNS6_PROTOCOL_GUID
pub const DNS6_PROTOCOL_GUID: Guid = Guid::new(
    0xca37bc1f,
    0xa327,
    0x4ae9,
    [0x82, 0x8a, 0x8c, 0x40, 0xd8, 0x50, 0x6a, 0x17],
);

/// IP4 Configuration Data
#[repr(C)]
pub struct Ip4ConfigData {
    pub default_protocol: Uint8,
    pub accept_any_protocol: Boolean,
    pub accept_icmp_errors: Boolean,
    pub accept_broadcast: Boolean,
    pub accept_promiscuous: Boolean,
    pub use_default_address: Boolean,
    pub station_address: Ipv4Address,
    pub subnet_mask: Ipv4Address,
    pub type_of_service: Uint8,
    pub time_to_live: Uint8,
    pub do_not_fragment: Boolean,
    pub raw_data: Boolean,
    pub receive_timeout: Uint32,
    pub transmit_timeout: Uint32,
}

/// IP6 Configuration Data
#[repr(C)]
pub struct Ip6ConfigData {
    pub default_protocol: Uint8,
    pub accept_any_protocol: Boolean,
    pub accept_icmp_errors: Boolean,
    pub accept_promiscuous: Boolean,
    pub destination_address: Ipv6Address,
    pub station_address: Ipv6Address,
    pub traffic_class: Uint8,
    pub hop_limit: Uint8,
    pub flow_label: Uint32,
    pub receive_timeout: Uint32,
    pub transmit_timeout: Uint32,
}

/// IP4 Mode Data
#[repr(C)]
pub struct Ip4ModeData {
    pub is_started: Boolean,
    pub max_packet_size: Uint32,
    pub config_data: Ip4ConfigData,
    pub is_configured: Boolean,
    pub group_count: Uint32,
    pub group_table: *mut Ipv4Address,
    pub route_count: Uint32,
    pub route_table: *mut Ip4RouteTable,
    pub icmp_type_count: Uint32,
    pub icmp_type_list: *mut Ip4IcmpType,
}

/// IP6 Mode Data
#[repr(C)]
pub struct Ip6ModeData {
    pub is_started: Boolean,
    pub max_packet_size: Uint32,
    pub config_data: Ip6ConfigData,
    pub is_configured: Boolean,
    pub address_count: Uint32,
    pub address_list: *mut Ip6AddressInfo,
    pub group_count: Uint32,
    pub group_table: *mut Ipv6Address,
    pub route_count: Uint32,
    pub route_table: *mut Ip6RouteTable,
    pub neighbor_count: Uint32,
    pub neighbor_cache: *mut Ip6NeighborCache,
    pub prefix_count: Uint32,
    pub prefix_table: *mut Ip6AddressInfo,
    pub icmp_type_count: Uint32,
    pub icmp_type_list: *mut Ip6IcmpType,
}

/// IP4 Route Table Entry
#[repr(C)]
pub struct Ip4RouteTable {
    pub subnet_address: Ipv4Address,
    pub subnet_mask: Ipv4Address,
    pub gateway_address: Ipv4Address,
}

/// IP6 Route Table Entry
#[repr(C)]
pub struct Ip6RouteTable {
    pub gateway: Ipv6Address,
    pub destination: Ipv6Address,
    pub prefix_length: Uint8,
}

/// IP6 Address Info
#[repr(C)]
pub struct Ip6AddressInfo {
    pub address: Ipv6Address,
    pub prefix_length: Uint8,
}

/// IP6 Neighbor Cache
#[repr(C)]
pub struct Ip6NeighborCache {
    pub neighbor: Ipv6Address,
    pub link_address: [Uint8; 32],
    pub state: Ip6NeighborState,
}

/// IP6 Neighbor State
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ip6NeighborState {
    Incomplete = 0,
    Reachable = 1,
    Stale = 2,
    Delay = 3,
    Probe = 4,
}

/// IP4 ICMP Type
#[repr(C)]
pub struct Ip4IcmpType {
    pub icmp_type: Uint8,
    pub icmp_code: Uint8,
}

/// IP6 ICMP Type
#[repr(C)]
pub struct Ip6IcmpType {
    pub icmp_type: Uint8,
    pub icmp_code: Uint8,
}

/// IP4 Header
#[repr(C)]
pub struct Ip4Header {
    pub header_length: Uint8,
    pub type_of_service: Uint8,
    pub total_length: Uint16,
    pub identification: Uint16,
    pub fragmentation: Uint16,
    pub time_to_live: Uint8,
    pub protocol: Uint8,
    pub checksum: Uint16,
    pub source_address: Ipv4Address,
    pub destination_address: Ipv4Address,
}

/// IP4 Fragment Data
#[repr(C)]
pub struct Ip4FragmentData {
    pub fragment_length: Uint32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

/// IP6 Fragment Data
#[repr(C)]
pub struct Ip6FragmentData {
    pub fragment_length: Uint32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

/// IP4 Receive Data
#[repr(C)]
pub struct Ip4ReceiveData {
    pub time_stamp: Time,
    pub recycle_signal: Event,
    pub header_length: Uint32,
    pub header: *mut Ip4Header,
    pub options_length: Uint32,
    pub options: *mut core::ffi::c_void,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Ip4FragmentData; 1],
}

/// IP6 Receive Data
#[repr(C)]
pub struct Ip6ReceiveData {
    pub time_stamp: Time,
    pub recycle_signal: Event,
    pub header_length: Uint32,
    pub header: *mut core::ffi::c_void,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Ip6FragmentData; 1],
}

/// IP4 Transmit Data
#[repr(C)]
pub struct Ip4TransmitData {
    pub destination_address: Ipv4Address,
    pub override_data: *mut Ip4OverrideData,
    pub options_length: Uint32,
    pub options_buffer: *mut core::ffi::c_void,
    pub total_data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Ip4FragmentData; 1],
}

/// IP6 Transmit Data
#[repr(C)]
pub struct Ip6TransmitData {
    pub destination_address: Ipv6Address,
    pub override_data: *mut Ip6OverrideData,
    pub ext_hdrs_length: Uint32,
    pub ext_hdrs: *mut core::ffi::c_void,
    pub next_header: Uint8,
    pub data_length: Uint32,
    pub fragment_count: Uint32,
    pub fragment_table: [Ip6FragmentData; 1],
}

/// IP4 Override Data
#[repr(C)]
pub struct Ip4OverrideData {
    pub source_address: Ipv4Address,
    pub gateway_address: Ipv4Address,
    pub protocol: Uint8,
    pub type_of_service: Uint8,
    pub time_to_live: Uint8,
    pub do_not_fragment: Boolean,
}

/// IP6 Override Data
#[repr(C)]
pub struct Ip6OverrideData {
    pub protocol: Uint8,
    pub hop_limit: Uint8,
    pub flow_label: Uint32,
}

/// IP4 Completion Token
#[repr(C)]
pub struct Ip4CompletionToken {
    pub event: Event,
    pub status: Status,
    pub packet: Ip4CompletionTokenPacket,
}

/// IP6 Completion Token
#[repr(C)]
pub struct Ip6CompletionToken {
    pub event: Event,
    pub status: Status,
    pub packet: Ip6CompletionTokenPacket,
}

/// IP4 Completion Token Packet Union
#[repr(C)]
pub union Ip4CompletionTokenPacket {
    pub rx_data: *mut Ip4ReceiveData,
    pub tx_data: *mut Ip4TransmitData,
}

/// IP6 Completion Token Packet Union
#[repr(C)]
pub union Ip6CompletionTokenPacket {
    pub rx_data: *mut Ip6ReceiveData,
    pub tx_data: *mut Ip6TransmitData,
}

/// EFI_IP4_PROTOCOL
#[repr(C)]
pub struct Ip4Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Ip4Protocol,
        ip4_mode_data: *mut Ip4ModeData,
        mnp_config_data: *mut core::ffi::c_void,
        snp_mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Ip4Protocol,
        ip4_config_data: *const Ip4ConfigData,
    ) -> Status,
    pub groups: unsafe extern "efiapi" fn(
        this: *mut Ip4Protocol,
        join_flag: Boolean,
        group_address: *const Ipv4Address,
    ) -> Status,
    pub routes: unsafe extern "efiapi" fn(
        this: *mut Ip4Protocol,
        delete_route: Boolean,
        subnet_address: *const Ipv4Address,
        subnet_mask: *const Ipv4Address,
        gateway_address: *const Ipv4Address,
    ) -> Status,
    pub transmit:
        unsafe extern "efiapi" fn(this: *mut Ip4Protocol, token: *mut Ip4CompletionToken) -> Status,
    pub receive:
        unsafe extern "efiapi" fn(this: *mut Ip4Protocol, token: *mut Ip4CompletionToken) -> Status,
    pub cancel:
        unsafe extern "efiapi" fn(this: *mut Ip4Protocol, token: *mut Ip4CompletionToken) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Ip4Protocol) -> Status,
}

/// EFI_IP6_PROTOCOL
#[repr(C)]
pub struct Ip6Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Ip6Protocol,
        ip6_mode_data: *mut Ip6ModeData,
        mnp_config_data: *mut core::ffi::c_void,
        snp_mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Ip6Protocol,
        ip6_config_data: *const Ip6ConfigData,
    ) -> Status,
    pub groups: unsafe extern "efiapi" fn(
        this: *mut Ip6Protocol,
        join_flag: Boolean,
        group_address: *const Ipv6Address,
    ) -> Status,
    pub routes: unsafe extern "efiapi" fn(
        this: *mut Ip6Protocol,
        delete_route: Boolean,
        destination: *const Ipv6Address,
        prefix_length: Uint8,
        gateway_address: *const Ipv6Address,
    ) -> Status,
    pub neighbors: unsafe extern "efiapi" fn(
        this: *mut Ip6Protocol,
        delete_flag: Boolean,
        target_ip6_address: *const Ipv6Address,
        target_link_address: *const [Uint8; 32],
        timeout: Uint32,
        override_flag: Boolean,
    ) -> Status,
    pub transmit:
        unsafe extern "efiapi" fn(this: *mut Ip6Protocol, token: *mut Ip6CompletionToken) -> Status,
    pub receive:
        unsafe extern "efiapi" fn(this: *mut Ip6Protocol, token: *mut Ip6CompletionToken) -> Status,
    pub cancel:
        unsafe extern "efiapi" fn(this: *mut Ip6Protocol, token: *mut Ip6CompletionToken) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Ip6Protocol) -> Status,
}

/// ARP Configuration Data
#[repr(C)]
pub struct ArpConfigData {
    pub sw_address_type: Uint16,
    pub sw_address_length: Uint8,
    pub station_address: *mut core::ffi::c_void,
    pub entry_timeout: Uint32,
    pub retry_count: Uint32,
    pub retry_timeout: Uint32,
}

/// EFI_ARP_PROTOCOL
#[repr(C)]
pub struct ArpProtocol {
    pub configure: unsafe extern "efiapi" fn(
        this: *mut ArpProtocol,
        config_data: *const ArpConfigData,
    ) -> Status,
    pub add: unsafe extern "efiapi" fn(
        this: *mut ArpProtocol,
        deny_flag: Boolean,
        target_sw_address: *const core::ffi::c_void,
        target_hw_address: *const core::ffi::c_void,
        timeout_value: Uint32,
        overwrite: Boolean,
    ) -> Status,
    pub find: unsafe extern "efiapi" fn(
        this: *mut ArpProtocol,
        by_sw_address: Boolean,
        address_buffer: *const core::ffi::c_void,
        entry_length: *mut Uint32,
        entry_count: *mut Uint32,
        entries: *mut *mut core::ffi::c_void,
        refresh: Boolean,
    ) -> Status,
    pub delete: unsafe extern "efiapi" fn(
        this: *mut ArpProtocol,
        by_sw_address: Boolean,
        address_buffer: *const core::ffi::c_void,
    ) -> Status,
    pub flush: unsafe extern "efiapi" fn(this: *mut ArpProtocol) -> Status,
    pub request: unsafe extern "efiapi" fn(
        this: *mut ArpProtocol,
        target_sw_address: *const core::ffi::c_void,
        resolved_event: Event,
        target_hw_address: *mut core::ffi::c_void,
    ) -> Status,
    pub cancel: unsafe extern "efiapi" fn(
        this: *mut ArpProtocol,
        target_sw_address: *const core::ffi::c_void,
        resolved_event: Event,
    ) -> Status,
}

/// DHCP4 State
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Dhcp4State {
    Stopped = 0,
    Init = 1,
    Selecting = 2,
    Requesting = 3,
    Bound = 4,
    Renewing = 5,
    Rebinding = 6,
    InitReboot = 7,
    Rebooting = 8,
}

/// DHCP4 Configuration Data
#[repr(C)]
pub struct Dhcp4ConfigData {
    pub discover_try_count: Uint32,
    pub discover_timeout: *mut Uint32,
    pub request_try_count: Uint32,
    pub request_timeout: *mut Uint32,
    pub client_address: Ipv4Address,
    pub dhcp4_callback: *mut core::ffi::c_void,
    pub callback_context: *mut core::ffi::c_void,
    pub option_count: Uint32,
    pub option_list: *mut *mut core::ffi::c_void,
}

/// EFI_DHCP4_PROTOCOL
#[repr(C)]
pub struct Dhcp4Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Dhcp4Protocol,
        mode_data: *mut core::ffi::c_void,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Dhcp4Protocol,
        config_data: *const Dhcp4ConfigData,
    ) -> Status,
    pub start:
        unsafe extern "efiapi" fn(this: *mut Dhcp4Protocol, completion_event: Event) -> Status,
    pub renew_rebind: unsafe extern "efiapi" fn(
        this: *mut Dhcp4Protocol,
        rebind_request: Boolean,
        completion_event: Event,
    ) -> Status,
    pub release: unsafe extern "efiapi" fn(this: *mut Dhcp4Protocol) -> Status,
    pub stop: unsafe extern "efiapi" fn(this: *mut Dhcp4Protocol) -> Status,
    pub build: unsafe extern "efiapi" fn(
        this: *mut Dhcp4Protocol,
        seed_packet: *mut core::ffi::c_void,
        delete_count: Uint32,
        delete_list: *mut Uint8,
        append_count: Uint32,
        append_list: *mut *mut core::ffi::c_void,
        new_packet: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub transmit_receive: unsafe extern "efiapi" fn(
        this: *mut Dhcp4Protocol,
        token: *mut core::ffi::c_void,
    ) -> Status,
    pub parse: unsafe extern "efiapi" fn(
        this: *mut Dhcp4Protocol,
        packet: *mut core::ffi::c_void,
        option_count: *mut Uint32,
        option_list: *mut *mut core::ffi::c_void,
    ) -> Status,
}

/// DNS Host Address
#[repr(C)]
pub struct DnsHostAddress {
    pub addr: [Uint8; 16],
}

/// DNS Configuration Data
#[repr(C)]
pub struct Dns4ConfigData {
    pub dns_server_list_count: Uint32,
    pub dns_server_list: *mut Ipv4Address,
    pub use_default_setting: Boolean,
    pub enable_dns_cache: Boolean,
    pub protocol: Uint8,
    pub station_ip: Ipv4Address,
    pub subnet_mask: Ipv4Address,
    pub local_port: Uint16,
    pub retry_count: Uint32,
    pub retry_interval: Uint32,
}

/// EFI_DNS4_PROTOCOL
#[repr(C)]
pub struct Dns4Protocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut Dns4Protocol,
        dns_mode_data: *mut Dns4ConfigData,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut Dns4Protocol,
        dns_config_data: *const Dns4ConfigData,
    ) -> Status,
    pub host_name_to_ip: unsafe extern "efiapi" fn(
        this: *mut Dns4Protocol,
        host_name: *const Char16,
        token: *mut core::ffi::c_void,
    ) -> Status,
    pub ip_to_host_name: unsafe extern "efiapi" fn(
        this: *mut Dns4Protocol,
        ip_address: Ipv4Address,
        token: *mut core::ffi::c_void,
    ) -> Status,
    pub general_lookup: unsafe extern "efiapi" fn(
        this: *mut Dns4Protocol,
        q_name: *const Char8,
        q_type: Uint16,
        q_class: Uint16,
        token: *mut core::ffi::c_void,
    ) -> Status,
    pub update_dns_cache: unsafe extern "efiapi" fn(
        this: *mut Dns4Protocol,
        delete_flag: Boolean,
        override_flag: Boolean,
        dns_cache_entry: *mut core::ffi::c_void,
    ) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut Dns4Protocol) -> Status,
    pub cancel:
        unsafe extern "efiapi" fn(this: *mut Dns4Protocol, token: *mut core::ffi::c_void) -> Status,
}

impl Ip4Protocol {
    /// Configure IP4 instance
    pub unsafe fn configure(&mut self, config: Option<&Ip4ConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Transmit packet
    pub unsafe fn transmit(&mut self, token: &mut Ip4CompletionToken) -> Status {
        (self.transmit)(self, token as *mut _)
    }

    /// Receive packet
    pub unsafe fn receive(&mut self, token: &mut Ip4CompletionToken) -> Status {
        (self.receive)(self, token as *mut _)
    }
}

impl Ip6Protocol {
    /// Configure IP6 instance
    pub unsafe fn configure(&mut self, config: Option<&Ip6ConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Transmit packet
    pub unsafe fn transmit(&mut self, token: &mut Ip6CompletionToken) -> Status {
        (self.transmit)(self, token as *mut _)
    }

    /// Receive packet
    pub unsafe fn receive(&mut self, token: &mut Ip6CompletionToken) -> Status {
        (self.receive)(self, token as *mut _)
    }
}
