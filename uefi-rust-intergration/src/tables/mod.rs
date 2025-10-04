// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Configuration Tables and Firmware Tables

pub mod acpi;
pub mod smbios;
pub mod configuration;
pub mod acpi_advanced;

pub use acpi::*;
pub use smbios::*;
pub use configuration::*;
pub use acpi_advanced::*;
