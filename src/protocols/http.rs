// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI HTTP Protocol

use crate::ffi::*;

/// EFI_HTTP_PROTOCOL_GUID
pub const HTTP_PROTOCOL_GUID: Guid = Guid::new(
    0x7A59B29B,
    0x910B,
    0x4171,
    [0x82, 0x42, 0xA8, 0x5A, 0x0D, 0xF2, 0x5B, 0x5B],
);

/// EFI_HTTP_SERVICE_BINDING_PROTOCOL_GUID
pub const HTTP_SERVICE_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0xbdc8e6af,
    0xd9bc,
    0x4379,
    [0xa7, 0x2a, 0xe0, 0xc4, 0xe7, 0x5d, 0xae, 0x1c],
);

/// HTTP Version
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HttpVersion {
    Http10 = 0,
    Http11 = 1,
    Http20 = 2,
    HttpVersionUnsupported = 3,
}

/// HTTP Method
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    HttpMethodGet = 0,
    HttpMethodPost = 1,
    HttpMethodPatch = 2,
    HttpMethodOptions = 3,
    HttpMethodConnect = 4,
    HttpMethodHead = 5,
    HttpMethodPut = 6,
    HttpMethodDelete = 7,
    HttpMethodTrace = 8,
    HttpMethodMax = 9,
}

/// HTTP Status Code
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HttpStatusCode {
    Http100Continue = 0,
    Http200Ok = 1,
    Http201Created = 2,
    Http202Accepted = 3,
    Http301MovedPermanently = 4,
    Http302Found = 5,
    Http304NotModified = 6,
    Http400BadRequest = 7,
    Http401Unauthorized = 8,
    Http403Forbidden = 9,
    Http404NotFound = 10,
    Http500InternalServerError = 11,
    Http501NotImplemented = 12,
    Http503ServiceUnavailable = 13,
    HttpStatusUnsupported = 14,
}

/// EFI_HTTP_HEADER
#[repr(C)]
pub struct HttpHeader {
    pub field_name: *mut Char8,
    pub field_value: *mut Char8,
}

/// EFI_HTTP_MESSAGE
#[repr(C)]
pub struct HttpMessage {
    pub data: HttpMessageData,
}

/// HTTP Message Data Union
#[repr(C)]
pub union HttpMessageData {
    pub request: *mut HttpRequestData,
    pub response: *mut HttpResponseData,
}

/// EFI_HTTP_REQUEST_DATA
#[repr(C)]
pub struct HttpRequestData {
    pub method: HttpMethod,
    pub url: *mut Char16,
}

/// EFI_HTTP_RESPONSE_DATA
#[repr(C)]
pub struct HttpResponseData {
    pub status_code: HttpStatusCode,
}

/// EFI_HTTP_TOKEN
#[repr(C)]
pub struct HttpToken {
    pub event: Event,
    pub status: Status,
    pub message: *mut HttpMessage,
}

/// EFI_HTTP_CONFIG_DATA
#[repr(C)]
pub struct HttpConfigData {
    pub http_version: HttpVersion,
    pub time_out_millisec: Uint32,
    pub local_addr_is_ipv6: Boolean,
    pub access_point: HttpConfigAccessPoint,
}

/// HTTP Config Access Point Union
#[repr(C)]
pub union HttpConfigAccessPoint {
    pub ipv4_node: *mut HttpConfigDataIpv4,
    pub ipv6_node: *mut HttpConfigDataIpv6,
}

/// IPv4 Configuration
#[repr(C)]
pub struct HttpConfigDataIpv4 {
    pub use_default_address: Boolean,
    pub local_ip: [Uint8; 4],
    pub local_subnet: [Uint8; 4],
    pub local_port: Uint16,
}

/// IPv6 Configuration
#[repr(C)]
pub struct HttpConfigDataIpv6 {
    pub local_ip: [Uint8; 16],
    pub local_port: Uint16,
}

/// EFI_HTTP_PROTOCOL
#[repr(C)]
pub struct HttpProtocol {
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *mut HttpProtocol,
        config_data: *mut HttpConfigData,
    ) -> Status,
    pub configure: unsafe extern "efiapi" fn(
        this: *mut HttpProtocol,
        config_data: *const HttpConfigData,
    ) -> Status,
    pub request:
        unsafe extern "efiapi" fn(this: *mut HttpProtocol, token: *mut HttpToken) -> Status,
    pub cancel: unsafe extern "efiapi" fn(this: *mut HttpProtocol, token: *mut HttpToken) -> Status,
    pub response:
        unsafe extern "efiapi" fn(this: *mut HttpProtocol, token: *mut HttpToken) -> Status,
    pub poll: unsafe extern "efiapi" fn(this: *mut HttpProtocol) -> Status,
}

impl HttpProtocol {
    /// Get current configuration
    pub unsafe fn get_mode_data(&mut self) -> Result<HttpConfigData, Status> {
        let mut config = core::mem::zeroed();
        let status = (self.get_mode_data)(self, &mut config);

        if status == EFI_SUCCESS {
            Ok(config)
        } else {
            Err(status)
        }
    }

    /// Configure the HTTP instance
    pub unsafe fn configure(&mut self, config: Option<&HttpConfigData>) -> Status {
        let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
        (self.configure)(self, config_ptr)
    }

    /// Send an HTTP request
    pub unsafe fn request(&mut self, token: &mut HttpToken) -> Status {
        (self.request)(self, token as *mut _)
    }

    /// Cancel a request
    pub unsafe fn cancel(&mut self, token: &mut HttpToken) -> Status {
        (self.cancel)(self, token as *mut _)
    }

    /// Receive an HTTP response
    pub unsafe fn response(&mut self, token: &mut HttpToken) -> Status {
        (self.response)(self, token as *mut _)
    }

    /// Poll for completion
    pub unsafe fn poll(&mut self) -> Status {
        (self.poll)(self)
    }
}

/// Helper functions for HTTP
pub mod http_helpers {
    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;

    /// Create a simple GET request
    pub unsafe fn create_get_request(url: &[u16]) -> HttpRequestData {
        HttpRequestData {
            method: HttpMethod::HttpMethodGet,
            url: url.as_ptr() as *mut Char16,
        }
    }

    /// Create a simple POST request
    pub unsafe fn create_post_request(url: &[u16]) -> HttpRequestData {
        HttpRequestData {
            method: HttpMethod::HttpMethodPost,
            url: url.as_ptr() as *mut Char16,
        }
    }

    /// Parse HTTP status code to integer
    pub fn status_code_to_int(code: HttpStatusCode) -> u32 {
        match code {
            HttpStatusCode::Http100Continue => 100,
            HttpStatusCode::Http200Ok => 200,
            HttpStatusCode::Http201Created => 201,
            HttpStatusCode::Http202Accepted => 202,
            HttpStatusCode::Http301MovedPermanently => 301,
            HttpStatusCode::Http302Found => 302,
            HttpStatusCode::Http304NotModified => 304,
            HttpStatusCode::Http400BadRequest => 400,
            HttpStatusCode::Http401Unauthorized => 401,
            HttpStatusCode::Http403Forbidden => 403,
            HttpStatusCode::Http404NotFound => 404,
            HttpStatusCode::Http500InternalServerError => 500,
            HttpStatusCode::Http501NotImplemented => 501,
            HttpStatusCode::Http503ServiceUnavailable => 503,
            _ => 0,
        }
    }
}
