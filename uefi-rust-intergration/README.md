# UEFI Rust Integration

A BSD-2-Clause-Patent licensed implementation of Rust support for UEFI firmware development, compatible with TianoCore EDK II.

## Features

- **No_std Runtime**: Custom Rust runtime without standard library dependencies
- **UEFI FFI Bindings**: Complete raw bindings to UEFI Boot Services and Runtime Services
- **Global Allocator**: Memory allocation using UEFI's AllocatePool/FreePool
- **Safe Abstractions**: Rust-friendly wrappers around unsafe UEFI APIs
- **BSD-2-Clause-Patent License**: Compatible with TianoCore licensing requirements

## Quick Start

### Prerequisites

- Rust nightly toolchain (`nightly-2025-01-09` or compatible)
- `x86_64-unknown-uefi` target installed

### Setup

```bash
# Install the required Rust toolchain
rustup toolchain install nightly-2025-01-09

# Add the UEFI target
rustup target add x86_64-unknown-uefi --toolchain nightly-2025-01-09

# Add rust-src component for building std library components
rustup component add rust-src --toolchain nightly-2025-01-09
```

### Building

```bash
# Build the library (recommended first to verify setup)
cargo build --lib

# Build the UEFI application
cargo build --release

# The output will be at:
# target/x86_64-unknown-uefi/release/uefi-app.efi
```

**Note**: The library currently builds successfully. Example binaries may have errors that need to be addressed.

## Example

```rust
#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: *mut Handle,
    system_table: *mut SystemTable,
) -> Status {
    unsafe {
        let st = &*system_table;
        let bs = st.boot_services();

        // Initialize allocator
        allocator::init_allocator(bs);

        // Use Rust collections
        let mut vec = Vec::new();
        vec.push(42);

        EFI_SUCCESS
    }
}
```

## Documentation

See [BUILD.md](BUILD.md) for detailed build instructions and project structure.

## Status

This is an experimental implementation addressing the [TianoCore task to add Rust support to EDK II](https://github.com/tianocore/tianocore.github.io/wiki/Tasks-Add-Rust-Support-to-EDK-II).

**Completed Components:**
- ✅ No_std Rust runtime configuration
- ✅ UEFI FFI bindings (Boot Services, Runtime Services, System Table)
- ✅ Global allocator implementation
- ✅ Enhanced panic handler with console output
- ✅ Language items and compiler intrinsics
- ✅ Safe Rust wrappers
- ✅ 8 Major UEFI protocol implementations:
  - Simple Text Input/Output
  - Graphics Output Protocol (GOP)
  - Block I/O and Simple File System
  - Device Path and Loaded Image
  - PCI I/O Protocol
- ✅ String handling utilities (UCS-2/UTF-16)
- ✅ GUID management utilities
- ✅ Logging framework with multiple levels
- ✅ Example UEFI applications
- ✅ Comprehensive test suite with mocks

**In Progress (See IMPLEMENTATION_STATUS.md):**
- ⚠️ Network protocols (SNP, PXE, HTTP)
- ⚠️ USB and additional hardware protocols
- ⚠️ Security protocols (Secure Boot, TPM)
- ⚠️ Multi-architecture support (ARM64, 32-bit)
- ⚠️ ACPI/SMBIOS table parsing
- ⚠️ Driver model implementation
- ⚠️ EDK II BaseTools integration (optional)
- ⚠️ QEMU automated testing

**Project Status: ~30% Complete**
- 8 of 50+ protocols implemented
- Core infrastructure solid
- Testing framework operational
- Ready for community contributions

## License

BSD-2-Clause-Patent

## Contributing

Contributions are welcome! Please ensure all code maintains:
- BSD-2-Clause-Patent licensing
- Compliance with UEFI Specification 2.10
- No_std compatibility
- Proper documentation
