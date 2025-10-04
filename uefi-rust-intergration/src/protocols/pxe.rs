// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI PXE (Preboot eXecution Environment) Base Code Protocol

use crate::ffi::*;
use crate::protocols::tcp_udp::Ipv4Address;

/// EFI_PXE_BASE_CODE_PROTOCOL_GUID
pub const PXE_BASE_CODE_PROTOCOL_GUID: Guid = Guid::new(
    0x03c4e603,
    0xac28,
    0x11d3,
    [0x9a, 0x2d, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

/// EFI_PXE_BASE_CODE_CALLBACK_PROTOCOL_GUID
pub const PXE_BASE_CODE_CALLBACK_PROTOCOL_GUID: Guid = Guid::new(
    0x245dca21,
    0xfb7b,
    0x11d3,
    [0x8f, 0x01, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// PXE Packet Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PxePacketType {
    Dhcp = 0,
    ProxyDhcp = 1,
}

/// PXE IP Address (can be IPv4 or IPv6)
#[repr(C)]
pub union PxeIpAddress {
    pub ipv4: Ipv4Address,
    pub ipv6: [Uint8; 16],
}

/// PXE MAC Address
#[repr(C)]
pub struct PxeMacAddress {
    pub addr: [Uint8; 32],
}

/// PXE DHCP Packet
#[repr(C)]
pub struct PxeDhcpPacket {
    pub bootp_opcode: Uint8,
    pub bootp_hw_type: Uint8,
    pub bootp_hw_addr_len: Uint8,
    pub bootp_gate_hops: Uint8,
    pub bootp_ident: Uint32,
    pub bootp_seconds: Uint16,
    pub bootp_flags: Uint16,
    pub bootp_ci_addr: [Uint8; 4],
    pub bootp_yi_addr: [Uint8; 4],
    pub bootp_si_addr: [Uint8; 4],
    pub bootp_gi_addr: [Uint8; 4],
    pub bootp_hw_addr: [Uint8; 16],
    pub bootp_srv_name: [Uint8; 64],
    pub bootp_boot_file: [Uint8; 128],
    pub dhcp_magik: Uint32,
    pub dhcp_options: [Uint8; 56],
}

/// PXE Mode Data
#[repr(C)]
pub struct PxeMode {
    pub started: Boolean,
    pub ipv6_available: Boolean,
    pub ipv6_supported: Boolean,
    pub using_ipv6: Boolean,
    pub bis_supported: Boolean,
    pub bis_detected: Boolean,
    pub auto_arp: Boolean,
    pub send_guid: Boolean,
    pub dhcp_discover_valid: Boolean,
    pub dhcp_ack_received: Boolean,
    pub proxy_offer_received: Boolean,
    pub pxe_discover_valid: Boolean,
    pub pxe_reply_received: Boolean,
    pub pxe_bis_reply_received: Boolean,
    pub icmp_error_received: Boolean,
    pub tftp_error_received: Boolean,
    pub make_callbacks: Boolean,
    pub ttl: Uint8,
    pub tos: Uint8,
    pub station_ip: PxeIpAddress,
    pub subnet_mask: PxeIpAddress,
    pub dhcp_discover: PxeDhcpPacket,
    pub dhcp_ack: PxeDhcpPacket,
    pub proxy_offer: PxeDhcpPacket,
    pub pxe_discover: PxeDhcpPacket,
    pub pxe_reply: PxeDhcpPacket,
    pub pxe_bis_reply: PxeDhcpPacket,
    pub ip_filter: PxeIpFilter,
    pub arp_cache_entries: Uint32,
    pub arp_cache: [PxeArpEntry; 8],
    pub route_table_entries: Uint32,
    pub route_table: [PxeRouteEntry; 8],
    pub icmp_error: PxeIcmpError,
    pub tftp_error: PxeTftpError,
}

/// PXE IP Filter
#[repr(C)]
pub struct PxeIpFilter {
    pub filters: Uint8,
    pub ip_count: Uint8,
    pub reserved: Uint16,
    pub ip_list: [PxeIpAddress; 8],
}

/// PXE ARP Cache Entry
#[repr(C)]
pub struct PxeArpEntry {
    pub ip_addr: PxeIpAddress,
    pub mac_addr: PxeMacAddress,
}

/// PXE Route Entry
#[repr(C)]
pub struct PxeRouteEntry {
    pub ip_addr: PxeIpAddress,
    pub subnet_mask: PxeIpAddress,
    pub gw_addr: PxeIpAddress,
}

/// PXE ICMP Error
#[repr(C)]
pub struct PxeIcmpError {
    pub error_type: Uint8,
    pub error_code: Uint8,
    pub reserved: [Uint16; 7],
    pub data: [Uint8; 494],
}

/// PXE TFTP Error
#[repr(C)]
pub struct PxeTftpError {
    pub error_code: Uint8,
    pub error_string: [Char8; 127],
}

/// MTFTP Information
#[repr(C)]
pub struct PxeMtftpInfo {
    pub mcast_ip: PxeIpAddress,
    pub cport: Uint16,
    pub sport: Uint16,
    pub listen_timeout: Uint16,
    pub transmit_timeout: Uint16,
}

/// EFI_PXE_BASE_CODE_PROTOCOL
#[repr(C)]
pub struct PxeBaseCodeProtocol {
    pub revision: Uint64,
    pub start: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        use_ipv6: Boolean,
    ) -> Status,
    pub stop: unsafe extern "efiapi" fn(this: *mut PxeBaseCodeProtocol) -> Status,
    pub dhcp: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        sort_offers: Boolean,
    ) -> Status,
    pub discover: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        boot_type: Uint16,
        layer: *mut Uint16,
        bis_reply: Boolean,
        info: *mut core::ffi::c_void,
    ) -> Status,
    pub mtftp: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        operation: Uint32,
        buffer_ptr: *mut core::ffi::c_void,
        overwrite: Boolean,
        buffer_size: *mut Uint64,
        block_size: *mut Uintn,
        server_ip: *const PxeIpAddress,
        filename: *const Char8,
        info: *const PxeMtftpInfo,
        dont_use_buffer: Boolean,
    ) -> Status,
    pub udp_write: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        op_flags: Uint16,
        dest_ip: *const PxeIpAddress,
        dest_port: *mut Uint16,
        gateway_ip: *const PxeIpAddress,
        src_ip: *const PxeIpAddress,
        src_port: *mut Uint16,
        header_size: *mut Uintn,
        header_ptr: *mut core::ffi::c_void,
        buffer_size: *mut Uintn,
        buffer_ptr: *mut core::ffi::c_void,
    ) -> Status,
    pub udp_read: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        op_flags: Uint16,
        dest_ip: *mut PxeIpAddress,
        dest_port: *mut Uint16,
        src_ip: *mut PxeIpAddress,
        src_port: *mut Uint16,
        header_size: *mut Uintn,
        header_ptr: *mut core::ffi::c_void,
        buffer_size: *mut Uintn,
        buffer_ptr: *mut core::ffi::c_void,
    ) -> Status,
    pub set_ip_filter: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        new_filter: *const PxeIpFilter,
    ) -> Status,
    pub arp: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        ip_addr: *const PxeIpAddress,
        mac_addr: *mut PxeMacAddress,
    ) -> Status,
    pub set_parameters: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        new_auto_arp: *const Boolean,
        new_send_guid: *const Boolean,
        new_ttl: *const Uint8,
        new_tos: *const Uint8,
        new_make_callback: *const Boolean,
    ) -> Status,
    pub set_station_ip: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        new_station_ip: *const PxeIpAddress,
        new_subnet_mask: *const PxeIpAddress,
    ) -> Status,
    pub set_packets: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeProtocol,
        new_dhcp_discover_valid: *const Boolean,
        new_dhcp_ack_received: *const Boolean,
        new_proxy_offer_received: *const Boolean,
        new_pxe_discover_valid: *const Boolean,
        new_pxe_reply_received: *const Boolean,
        new_pxe_bis_reply_received: *const Boolean,
        new_dhcp_discover: *const PxeDhcpPacket,
        new_dhcp_ack: *const PxeDhcpPacket,
        new_proxy_offer: *const PxeDhcpPacket,
        new_pxe_discover: *const PxeDhcpPacket,
        new_pxe_reply: *const PxeDhcpPacket,
        new_pxe_bis_reply: *const PxeDhcpPacket,
    ) -> Status,
    pub mode: *mut PxeMode,
}

/// PXE Callback Event Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PxeBaseCodeCallbackEvent {
    First = 0,
    Dhcp = 1,
    Discover = 2,
    MtftpGetFileSize = 3,
    MtftpReadDirectory = 4,
    MtftpReadFile = 5,
    MtftpSendFile = 6,
    MtftpReadFileReceived = 7,
    MtftpReadFileFailed = 8,
    MtftpReadFileDone = 9,
    SendFileReceived = 10,
    SendFileFailed = 11,
    SendFileDone = 12,
}

/// PXE Callback Status
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PxeBaseCodeCallbackStatus {
    Continue = 0,
    Abort = 1,
}

/// EFI_PXE_BASE_CODE_CALLBACK_PROTOCOL
#[repr(C)]
pub struct PxeBaseCodeCallbackProtocol {
    pub revision: Uint64,
    pub callback: unsafe extern "efiapi" fn(
        this: *mut PxeBaseCodeCallbackProtocol,
        function: PxeBaseCodeCallbackEvent,
        received: Boolean,
        packet_len: Uint32,
        packet: *const core::ffi::c_void,
    ) -> PxeBaseCodeCallbackStatus,
}

impl PxeBaseCodeProtocol {
    /// Start the PXE Base Code
    pub unsafe fn start(&mut self, use_ipv6: bool) -> Status {
        (self.start)(self, use_ipv6 as Boolean)
    }

    /// Stop the PXE Base Code
    pub unsafe fn stop(&mut self) -> Status {
        (self.stop)(self)
    }

    /// Perform DHCP discovery
    pub unsafe fn dhcp(&mut self, sort_offers: bool) -> Status {
        (self.dhcp)(self, sort_offers as Boolean)
    }

    /// Perform PXE server discovery
    pub unsafe fn discover(
        &mut self,
        boot_type: u16,
        layer: Option<&mut u16>,
        bis_reply: bool,
    ) -> Status {
        let layer_ptr = layer
            .map(|l| l as *mut _)
            .unwrap_or(core::ptr::null_mut());
        (self.discover)(
            self,
            boot_type,
            layer_ptr,
            bis_reply as Boolean,
            core::ptr::null_mut(),
        )
    }

    /// Read data via UDP
    pub unsafe fn udp_read(
        &mut self,
        dest_port: &mut u16,
        src_ip: &mut PxeIpAddress,
        src_port: &mut u16,
        buffer: &mut [u8],
    ) -> Result<usize, Status> {
        let mut buffer_size = buffer.len();
        let mut dest_ip = core::mem::zeroed();

        let status = (self.udp_read)(
            self,
            0,
            &mut dest_ip,
            dest_port as *mut _,
            src_ip as *mut _,
            src_port as *mut _,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut buffer_size,
            buffer.as_mut_ptr() as *mut _,
        );

        if status == EFI_SUCCESS {
            Ok(buffer_size)
        } else {
            Err(status)
        }
    }

    /// Write data via UDP
    pub unsafe fn udp_write(
        &mut self,
        dest_ip: &PxeIpAddress,
        dest_port: u16,
        buffer: &[u8],
    ) -> Result<usize, Status> {
        let mut buffer_size = buffer.len();
        let mut port = dest_port;

        let status = (self.udp_write)(
            self,
            0,
            dest_ip as *const _,
            &mut port,
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut buffer_size,
            buffer.as_ptr() as *mut _,
        );

        if status == EFI_SUCCESS {
            Ok(buffer_size)
        } else {
            Err(status)
        }
    }

    /// Resolve IP address to MAC address via ARP
    pub unsafe fn arp(&mut self, ip_addr: &PxeIpAddress) -> Result<PxeMacAddress, Status> {
        let mut mac_addr: PxeMacAddress = core::mem::zeroed();
        let status = (self.arp)(self, ip_addr as *const _, &mut mac_addr);

        if status == EFI_SUCCESS {
            Ok(mac_addr)
        } else {
            Err(status)
        }
    }

    /// Get mode data
    pub unsafe fn get_mode(&self) -> Option<&PxeMode> {
        if self.mode.is_null() {
            None
        } else {
            Some(&*self.mode)
        }
    }
}

/// TFTP OpCodes
pub const PXE_TFTP_OPCODE_RRQ: u16 = 1;
pub const PXE_TFTP_OPCODE_WRQ: u16 = 2;
pub const PXE_TFTP_OPCODE_DATA: u16 = 3;
pub const PXE_TFTP_OPCODE_ACK: u16 = 4;
pub const PXE_TFTP_OPCODE_ERROR: u16 = 5;
pub const PXE_TFTP_OPCODE_OACK: u16 = 6;

/// MTFTP Operations
pub const PXE_MTFTP_GET_FILE_SIZE: u32 = 0;
pub const PXE_MTFTP_READ_DIRECTORY: u32 = 1;
pub const PXE_MTFTP_READ_FILE: u32 = 2;
pub const PXE_MTFTP_WRITE_FILE: u32 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pxe_mode_size() {
        // Ensure PxeMode structure is properly sized
        assert!(core::mem::size_of::<PxeMode>() > 0);
    }

    #[test]
    fn test_tftp_opcodes() {
        assert_eq!(PXE_TFTP_OPCODE_RRQ, 1);
        assert_eq!(PXE_TFTP_OPCODE_DATA, 3);
        assert_eq!(PXE_TFTP_OPCODE_ERROR, 5);
    }
}
