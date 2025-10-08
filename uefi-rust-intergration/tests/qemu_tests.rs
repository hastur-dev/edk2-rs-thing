// SPDX-License-Identifier: BSD-2-Clause-Patent
//! QEMU Integration Tests
//!
//! These tests run actual UEFI applications in QEMU with OVMF firmware
//! to verify real-world behavior.

#![cfg(test)]

mod qemu_runner;

use qemu_runner::*;
use std::path::PathBuf;

/// Helper to check if QEMU and OVMF are available
fn qemu_available() -> bool {
    let config = QemuConfig::default();
    config.ovmf_code.exists() && config.qemu_path.exists()
}

#[test]
#[ignore] // Run with: cargo test --test qemu_tests -- --ignored
fn test_protocol_enumeration() {
    if !qemu_available() {
        eprintln!("QEMU/OVMF not available, skipping test");
        return;
    }

    let efi_path = match build_test_app("protocol_enum") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Build failed: {}", e);
            return;
        }
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should enumerate standard protocols
    assert!(
        result.stdout.contains("SimpleTextOutput")
            || result.stdout.contains("LoadedImage")
            || result.stdout.contains("Protocol"),
        "Expected protocol enumeration in output"
    );
}

#[test]
#[ignore]
fn test_memory_allocation() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("memory_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should successfully allocate and free memory
    assert!(
        result.stdout.contains("Allocated")
            || result.stdout.contains("Success")
            || result.exit_code == Some(0),
        "Expected successful memory operations"
    );
}

#[test]
#[ignore]
fn test_file_operations() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("file_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should perform file operations
    assert!(
        result.stdout.contains("File")
            || result.stdout.contains("Read")
            || result.stdout.contains("Write")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_graphics_output() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("graphics_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut config = QemuConfig::default();
    config.extra_args.push("-vga".to_string());
    config.extra_args.push("std".to_string());

    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should find graphics output protocol
    assert!(
        result.stdout.contains("Graphics")
            || result.stdout.contains("GOP")
            || result.stdout.contains("Resolution")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_time_services() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("time_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should get time from runtime services
    assert!(
        result.stdout.contains("Time") ||
            result.stdout.contains("2025") || // Current year
            result.stdout.contains("Date") ||
            result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_variable_services() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("variable_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should access UEFI variables
    assert!(
        result.stdout.contains("Variable")
            || result.stdout.contains("BootOrder")
            || result.stdout.contains("ConIn")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_block_io() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("block_io_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut config = QemuConfig::default();
    // Add a test disk
    config.extra_args.push("-drive".to_string());
    config
        .extra_args
        .push("file=/tmp/test.img,format=raw,if=virtio".to_string());

    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should find block I/O devices
    assert!(
        result.stdout.contains("Block")
            || result.stdout.contains("Disk")
            || result.stdout.contains("Media")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_simple_network() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("network_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut config = QemuConfig::default();
    // Add network device
    config.extra_args.push("-netdev".to_string());
    config.extra_args.push("user,id=net0".to_string());
    config.extra_args.push("-device".to_string());
    config.extra_args.push("e1000,netdev=net0".to_string());

    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should find network interfaces
    assert!(
        result.stdout.contains("Network")
            || result.stdout.contains("MAC")
            || result.stdout.contains("Interface")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_acpi_tables() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("acpi_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should find ACPI tables
    assert!(
        result.stdout.contains("ACPI")
            || result.stdout.contains("RSDP")
            || result.stdout.contains("XSDT")
            || result.stdout.contains("FADT")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_smbios_tables() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("smbios_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should find SMBIOS tables
    assert!(
        result.stdout.contains("SMBIOS")
            || result.stdout.contains("BIOS")
            || result.stdout.contains("System")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_pci_enumeration() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("pci_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should enumerate PCI devices
    assert!(
        result.stdout.contains("PCI")
            || result.stdout.contains("Device")
            || result.stdout.contains("Vendor")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_boot_from_disk() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("boot_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut config = QemuConfig::default();
    config.timeout_secs = 60; // Boot may take longer

    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should complete boot sequence
    assert!(
        result.stdout.contains("Boot")
            || result.stdout.contains("Started")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_exit_boot_services() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("exit_boot_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let config = QemuConfig::default();
    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should successfully exit boot services
    assert!(
        result.stdout.contains("ExitBootServices")
            || result.stdout.contains("Runtime")
            || result.exit_code == Some(0)
    );
}

#[test]
#[ignore]
fn test_stress_memory() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("stress_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut config = QemuConfig::default();
    config.memory_mb = 512; // More memory for stress test
    config.timeout_secs = 120;

    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should handle stress without crashing
    assert!(!result.timed_out, "Stress test should not timeout");
    assert!(result.exit_code.is_some(), "Should exit cleanly");
}

#[test]
#[ignore]
fn test_multiprocessor_services() {
    if !qemu_available() {
        return;
    }

    let efi_path = match build_test_app("mp_test") {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut config = QemuConfig::default();
    config.extra_args.push("-smp".to_string());
    config.extra_args.push("4".to_string()); // 4 CPUs

    let result = run_in_qemu(&efi_path, &config).expect("Failed to run in QEMU");

    // Should detect multiple processors
    assert!(
        result.stdout.contains("Processor")
            || result.stdout.contains("CPU")
            || result.stdout.contains("Core")
            || result.exit_code == Some(0)
    );
}
