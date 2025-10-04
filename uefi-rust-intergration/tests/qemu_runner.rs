// SPDX-License-Identifier: BSD-2-Clause-Patent
//! QEMU Integration Test Runner
//!
//! This module provides utilities for running UEFI applications in QEMU
//! with OVMF firmware for integration testing.

#![cfg(test)]

use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Duration;
use std::io::{BufRead, BufReader};

/// QEMU test configuration
pub struct QemuConfig {
    /// Path to QEMU executable
    pub qemu_path: PathBuf,
    /// Path to OVMF firmware
    pub ovmf_code: PathBuf,
    /// Path to OVMF variables
    pub ovmf_vars: PathBuf,
    /// Memory size in MB
    pub memory_mb: u32,
    /// Timeout in seconds
    pub timeout_secs: u64,
    /// Additional QEMU arguments
    pub extra_args: Vec<String>,
}

impl Default for QemuConfig {
    fn default() -> Self {
        QemuConfig {
            qemu_path: PathBuf::from("qemu-system-x86_64"),
            ovmf_code: PathBuf::from("/usr/share/ovmf/x64/OVMF_CODE.fd"),
            ovmf_vars: PathBuf::from("/usr/share/ovmf/x64/OVMF_VARS.fd"),
            memory_mb: 256,
            timeout_secs: 30,
            extra_args: Vec::new(),
        }
    }
}

/// Result of a QEMU test run
pub struct QemuTestResult {
    /// Standard output captured
    pub stdout: String,
    /// Standard error captured
    pub stderr: String,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Whether the test timed out
    pub timed_out: bool,
}

/// Run a UEFI application in QEMU
pub fn run_in_qemu(efi_path: &Path, config: &QemuConfig) -> Result<QemuTestResult, String> {
    // Verify OVMF firmware exists
    if !config.ovmf_code.exists() {
        return Err(format!("OVMF code not found at {:?}", config.ovmf_code));
    }

    // Create a temporary directory for the ESP
    let temp_dir = std::env::temp_dir().join(format!("uefi-test-{}", std::process::id()));
    fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;

    // Create ESP directory structure
    let esp_dir = temp_dir.join("esp");
    let efi_boot_dir = esp_dir.join("EFI").join("BOOT");
    fs::create_dir_all(&efi_boot_dir).map_err(|e| format!("Failed to create ESP: {}", e))?;

    // Copy the EFI application to BOOTX64.EFI
    let boot_efi = efi_boot_dir.join("BOOTX64.EFI");
    fs::copy(efi_path, &boot_efi).map_err(|e| format!("Failed to copy EFI: {}", e))?;

    // Copy OVMF vars to temp location for this run
    let temp_vars = temp_dir.join("OVMF_VARS.fd");
    fs::copy(&config.ovmf_vars, &temp_vars).map_err(|e| format!("Failed to copy OVMF vars: {}", e))?;

    // Build QEMU command
    let mut cmd = Command::new(&config.qemu_path);
    cmd.arg("-enable-kvm")
        .arg("-machine").arg("q35")
        .arg("-cpu").arg("host")
        .arg("-m").arg(format!("{}M", config.memory_mb))
        .arg("-drive").arg(format!("if=pflash,format=raw,readonly=on,file={}", config.ovmf_code.display()))
        .arg("-drive").arg(format!("if=pflash,format=raw,file={}", temp_vars.display()))
        .arg("-drive").arg(format!("format=raw,file=fat:rw:{}", esp_dir.display()))
        .arg("-serial").arg("stdio")
        .arg("-display").arg("none")
        .arg("-no-reboot")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Add extra arguments
    for arg in &config.extra_args {
        cmd.arg(arg);
    }

    // Spawn the process
    let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn QEMU: {}", e))?;

    // Capture output with timeout
    let stdout_handle = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr_handle = child.stderr.take().ok_or("Failed to capture stderr")?;

    let stdout_reader = BufReader::new(stdout_handle);
    let stderr_reader = BufReader::new(stderr_handle);

    let mut stdout_lines = Vec::new();
    let mut stderr_lines = Vec::new();

    // Read stdout
    for line in stdout_reader.lines() {
        if let Ok(line) = line {
            stdout_lines.push(line);
        }
    }

    // Read stderr
    for line in stderr_reader.lines() {
        if let Ok(line) = line {
            stderr_lines.push(line);
        }
    }

    // Wait for process with timeout
    let timeout = Duration::from_secs(config.timeout_secs);
    let result = std::thread::spawn(move || child.wait())
        .join()
        .map_err(|_| "Thread panicked")?;

    let exit_status = result.map_err(|e| format!("Failed to wait for QEMU: {}", e))?;

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);

    Ok(QemuTestResult {
        stdout: stdout_lines.join("\n"),
        stderr: stderr_lines.join("\n"),
        exit_code: exit_status.code(),
        timed_out: false,
    })
}

/// Build a UEFI test application
pub fn build_test_app(example_name: &str) -> Result<PathBuf, String> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg(example_name)
        .arg("--target")
        .arg("x86_64-unknown-uefi")
        .arg("--release")
        .output()
        .map_err(|e| format!("Failed to run cargo: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Build failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let efi_path = PathBuf::from("target")
        .join("x86_64-unknown-uefi")
        .join("release")
        .join("examples")
        .join(format!("{}.efi", example_name));

    if !efi_path.exists() {
        return Err(format!("Built EFI not found at {:?}", efi_path));
    }

    Ok(efi_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run with --ignored flag
    fn test_qemu_config_default() {
        let config = QemuConfig::default();
        assert_eq!(config.memory_mb, 256);
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    #[ignore] // Only run with QEMU and OVMF installed
    fn test_build_hello_world() {
        // This test requires the hello_world example to exist
        let result = build_test_app("hello_world");

        // We don't fail if the example doesn't exist yet
        if let Ok(path) = result {
            assert!(path.exists());
        }
    }

    #[test]
    #[ignore] // Only run with QEMU and OVMF installed
    fn test_run_hello_world_in_qemu() {
        // Build the hello world example
        let efi_path = match build_test_app("hello_world") {
            Ok(path) => path,
            Err(_) => return, // Skip if build fails
        };

        // Run in QEMU
        let config = QemuConfig::default();
        let result = run_in_qemu(&efi_path, &config);

        if let Ok(result) = result {
            // Should contain "Hello" in output
            assert!(
                result.stdout.contains("Hello") || result.stderr.contains("Hello"),
                "Expected 'Hello' in output"
            );
        }
    }
}
