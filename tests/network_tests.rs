// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Network Protocol Tests

#![cfg(test)]

use uefi_rust::ffi::*;
use uefi_rust::protocols::*;

#[test]
fn test_ipv4_address_creation() {
    use tcp_udp::Ipv4Address;

    let localhost = Ipv4Address {
        addr: [127, 0, 0, 1],
    };

    assert_eq!(localhost.addr, [127, 0, 0, 1]);

    let google_dns = Ipv4Address { addr: [8, 8, 8, 8] };

    assert_eq!(google_dns.addr, [8, 8, 8, 8]);
}

#[test]
fn test_ipv6_address_creation() {
    use tcp_udp::Ipv6Address;

    let localhost = Ipv6Address {
        addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    };

    assert_eq!(localhost.addr[15], 1);
    assert_eq!(localhost.addr[0..15], [0; 15]);
}

#[test]
fn test_tcp_connection_states() {
    use tcp_udp::Tcp4ConnectionState;

    let states = [
        Tcp4ConnectionState::Closed,
        Tcp4ConnectionState::Listen,
        Tcp4ConnectionState::SynSent,
        Tcp4ConnectionState::SynReceived,
        Tcp4ConnectionState::Established,
        Tcp4ConnectionState::FinWait1,
        Tcp4ConnectionState::FinWait2,
        Tcp4ConnectionState::Closing,
        Tcp4ConnectionState::TimeWait,
        Tcp4ConnectionState::CloseWait,
        Tcp4ConnectionState::LastAck,
    ];

    assert_eq!(states.len(), 11);
    assert_eq!(states[0] as u32, 0);
    assert_eq!(states[4] as u32, 4); // Established
}

#[test]
fn test_tcp_config_creation() {
    use tcp_udp::*;

    let config = Tcp4ConfigData {
        type_of_service: 0,
        time_to_live: 64,
        access_point: Tcp4AccessPoint {
            use_default_address: 1,
            station_address: Ipv4Address { addr: [0, 0, 0, 0] },
            subnet_mask: Ipv4Address { addr: [0, 0, 0, 0] },
            station_port: 0,
            remote_address: Ipv4Address {
                addr: [192, 168, 1, 1],
            },
            remote_port: 80,
            active_flag: 1,
        },
        control_option: core::ptr::null_mut(),
    };

    assert_eq!(config.time_to_live, 64);
    assert_eq!(config.access_point.remote_port, 80);
}

#[test]
fn test_udp_config_creation() {
    use tcp_udp::*;

    let config = Udp4ConfigData {
        accept_broadcast: 1,
        accept_promiscuous: 0,
        accept_any_port: 0,
        allow_duplicate_port: 0,
        type_of_service: 0,
        time_to_live: 64,
        do_not_fragment: 0,
        receive_timeout: 0,
        transmit_timeout: 0,
        use_default_address: 1,
        station_address: Ipv4Address { addr: [0, 0, 0, 0] },
        subnet_mask: Ipv4Address { addr: [0, 0, 0, 0] },
        station_port: 0,
        remote_address: Ipv4Address { addr: [0, 0, 0, 0] },
        remote_port: 0,
    };

    assert_eq!(config.accept_broadcast, 1);
    assert_eq!(config.time_to_live, 64);
}

#[test]
fn test_http_methods() {
    assert_eq!(HttpMethod::HttpMethodGet as u32, 0);
    assert_eq!(HttpMethod::HttpMethodPost as u32, 1);
    assert_eq!(HttpMethod::HttpMethodPatch as u32, 2);
    assert_eq!(HttpMethod::HttpMethodOptions as u32, 3);
    assert_eq!(HttpMethod::HttpMethodConnect as u32, 4);
    assert_eq!(HttpMethod::HttpMethodHead as u32, 5);
    assert_eq!(HttpMethod::HttpMethodPut as u32, 6);
    assert_eq!(HttpMethod::HttpMethodDelete as u32, 7);
    assert_eq!(HttpMethod::HttpMethodTrace as u32, 8);
}

#[test]
fn test_http_versions() {
    assert_eq!(HttpVersion::Http10 as u32, 0);
    assert_eq!(HttpVersion::Http11 as u32, 1);
    assert_eq!(HttpVersion::Http20 as u32, 2);
}

#[test]
fn test_http_status_codes() {
    use http::http_helpers::status_code_to_int;

    assert_eq!(status_code_to_int(HttpStatusCode::Http100Continue), 100);
    assert_eq!(status_code_to_int(HttpStatusCode::Http200Ok), 200);
    assert_eq!(status_code_to_int(HttpStatusCode::Http201Created), 201);
    assert_eq!(
        status_code_to_int(HttpStatusCode::Http301MovedPermanently),
        301
    );
    assert_eq!(status_code_to_int(HttpStatusCode::Http302Found), 302);
    assert_eq!(status_code_to_int(HttpStatusCode::Http400BadRequest), 400);
    assert_eq!(status_code_to_int(HttpStatusCode::Http404NotFound), 404);
    assert_eq!(
        status_code_to_int(HttpStatusCode::Http500InternalServerError),
        500
    );
}

#[test]
fn test_ip_protocol_numbers() {
    const IPPROTO_ICMP: u8 = 1;
    const IPPROTO_TCP: u8 = 6;
    const IPPROTO_UDP: u8 = 17;

    assert_eq!(IPPROTO_ICMP, 1);
    assert_eq!(IPPROTO_TCP, 6);
    assert_eq!(IPPROTO_UDP, 17);
}

#[test]
fn test_dhcp_state() {
    use ip::Dhcp4State;

    assert_eq!(Dhcp4State::Stopped as u32, 0);
    assert_eq!(Dhcp4State::Init as u32, 1);
    assert_eq!(Dhcp4State::Selecting as u32, 2);
    assert_eq!(Dhcp4State::Requesting as u32, 3);
    assert_eq!(Dhcp4State::Bound as u32, 4);
    assert_eq!(Dhcp4State::Renewing as u32, 5);
    assert_eq!(Dhcp4State::Rebinding as u32, 6);
}

#[test]
fn test_pxe_tftp_opcodes() {
    use pxe::*;

    assert_eq!(PXE_TFTP_OPCODE_RRQ, 1);
    assert_eq!(PXE_TFTP_OPCODE_WRQ, 2);
    assert_eq!(PXE_TFTP_OPCODE_DATA, 3);
    assert_eq!(PXE_TFTP_OPCODE_ACK, 4);
    assert_eq!(PXE_TFTP_OPCODE_ERROR, 5);
    assert_eq!(PXE_TFTP_OPCODE_OACK, 6);
}

#[test]
fn test_pxe_mtftp_operations() {
    use pxe::*;

    assert_eq!(PXE_MTFTP_GET_FILE_SIZE, 0);
    assert_eq!(PXE_MTFTP_READ_DIRECTORY, 1);
    assert_eq!(PXE_MTFTP_READ_FILE, 2);
    assert_eq!(PXE_MTFTP_WRITE_FILE, 3);
}

#[test]
fn test_network_mac_address() {
    use simple_network::MacAddress;

    let mac = MacAddress {
        addr: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
    };

    assert_eq!(mac.addr[0], 0x00);
    assert_eq!(mac.addr[5], 0x55);
}

#[test]
fn test_network_statistics() {
    use simple_network::NetworkStatistics;

    let mut stats: NetworkStatistics = unsafe { core::mem::zeroed() };
    stats.rx_total_frames = 1000;
    stats.tx_total_frames = 800;

    assert_eq!(stats.rx_total_frames, 1000);
    assert_eq!(stats.tx_total_frames, 800);
}

#[test]
fn test_ip_neighbor_state() {
    use ip::Ip6NeighborState;

    assert_eq!(Ip6NeighborState::Incomplete as u32, 0);
    assert_eq!(Ip6NeighborState::Reachable as u32, 1);
    assert_eq!(Ip6NeighborState::Stale as u32, 2);
    assert_eq!(Ip6NeighborState::Delay as u32, 3);
    assert_eq!(Ip6NeighborState::Probe as u32, 4);
}

#[test]
fn test_subnet_mask_validation() {
    use tcp_udp::Ipv4Address;

    // Class C subnet mask
    let mask = Ipv4Address {
        addr: [255, 255, 255, 0],
    };

    assert_eq!(mask.addr, [255, 255, 255, 0]);

    // Class B subnet mask
    let mask = Ipv4Address {
        addr: [255, 255, 0, 0],
    };

    assert_eq!(mask.addr, [255, 255, 0, 0]);
}

#[test]
fn test_port_numbers() {
    const HTTP_PORT: u16 = 80;
    const HTTPS_PORT: u16 = 443;
    const DNS_PORT: u16 = 53;
    const DHCP_SERVER_PORT: u16 = 67;
    const DHCP_CLIENT_PORT: u16 = 68;

    assert_eq!(HTTP_PORT, 80);
    assert_eq!(HTTPS_PORT, 443);
    assert_eq!(DNS_PORT, 53);
    assert_eq!(DHCP_SERVER_PORT, 67);
    assert_eq!(DHCP_CLIENT_PORT, 68);
}

#[test]
fn test_network_protocol_guids_unique() {
    use http::*;
    use ip::*;
    use tcp_udp::*;

    // Verify all network protocol GUIDs are unique
    assert_ne!(TCP4_PROTOCOL_GUID, TCP6_PROTOCOL_GUID);
    assert_ne!(UDP4_PROTOCOL_GUID, UDP6_PROTOCOL_GUID);
    assert_ne!(IP4_PROTOCOL_GUID, IP6_PROTOCOL_GUID);
    assert_ne!(TCP4_PROTOCOL_GUID, UDP4_PROTOCOL_GUID);
    assert_ne!(HTTP_PROTOCOL_GUID, TCP4_PROTOCOL_GUID);
}

#[test]
fn test_service_binding_guids_unique() {
    use tcp_udp::*;

    assert_ne!(TCP4_SERVICE_BINDING_PROTOCOL_GUID, TCP4_PROTOCOL_GUID);
    assert_ne!(TCP6_SERVICE_BINDING_PROTOCOL_GUID, TCP6_PROTOCOL_GUID);
    assert_ne!(UDP4_SERVICE_BINDING_PROTOCOL_GUID, UDP4_PROTOCOL_GUID);
    assert_ne!(UDP6_SERVICE_BINDING_PROTOCOL_GUID, UDP6_PROTOCOL_GUID);
}
