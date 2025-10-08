// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Protocol definitions

pub mod block_io;
pub mod device_path;
pub mod driver_binding;
pub mod firmware_management;
pub mod graphics_output;
pub mod hii;
pub mod http;
pub mod ip;
pub mod loaded_image;
pub mod misc;
pub mod mp_services;
pub mod pci_io;
pub mod pxe;
pub mod security;
pub mod shell;
pub mod simple_file_system;
pub mod simple_network;
pub mod simple_text_input;
pub mod simple_text_output;
pub mod storage;
pub mod tcp_udp;
pub mod usb_io;

pub use block_io::*;
pub use device_path::*;
pub use driver_binding::*;
pub use firmware_management::*;
pub use graphics_output::*;
pub use hii::*;
pub use http::*;
pub use ip::*;
pub use loaded_image::*;
pub use misc::*;
pub use mp_services::*;
pub use pci_io::*;
pub use pxe::*;
pub use security::*;
// Note: shell::* and simple_file_system::* have overlapping exports (EFI_FILE_* constants)
// Re-export simple_file_system which has the canonical definitions
pub use simple_file_system::*;
pub use simple_network::*;
pub use simple_text_input::*;
pub use simple_text_output::*;
pub use storage::*;
pub use tcp_udp::*;
pub use usb_io::*;
