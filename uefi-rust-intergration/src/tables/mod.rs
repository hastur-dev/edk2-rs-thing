// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Configuration Tables and Firmware Tables

pub mod acpi;
pub mod acpi_advanced;
pub mod configuration;
pub mod smbios;

pub use acpi::*;
pub use acpi_advanced::*;
pub use configuration::*;
pub use smbios::*;
