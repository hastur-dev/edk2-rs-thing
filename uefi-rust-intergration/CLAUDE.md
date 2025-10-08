# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a BSD-2-Clause-Patent licensed UEFI Rust implementation for firmware development, compatible with TianoCore EDK II. The project provides a complete no_std runtime with UEFI FFI bindings and safe Rust abstractions.

**Status**: ~90% complete (15,000+ lines, 23 protocols with safe wrappers, 60+ source files, 1,400+ tests)

## Build Commands

### Prerequisites
```bash
# Install required toolchain
rustup toolchain install nightly-2025-01-09
rustup target add x86_64-unknown-uefi --toolchain nightly-2025-01-09
rustup component add rust-src --toolchain nightly-2025-01-09
```

### Standard Build Commands
```bash
# Build library only (recommended first step to verify setup)
cargo build --lib

# Build library with nightly-2025-01-09 explicitly
cargo +nightly-2025-01-09 build --lib

# Build UEFI application for release
cargo build --release

# Build specific binary
cargo build --bin uefi-app --release
cargo build --bin hello-protocols --release
```

Output location: `target/x86_64-unknown-uefi/release/*.efi`

### Testing
```bash
# Run all tests (native, uses mock UEFI environment)
cargo test

# Run specific test file
cargo test --test ffi_tests

# Run with verbose output
cargo test -- --nocapture

# Test specific module pattern
cargo test allocator
```

### Multi-Architecture Testing
```bash
# Test ARM64 compilation
cargo build --lib --target aarch64-unknown-uefi

# Test i686 compilation
cargo build --lib --target i686-unknown-uefi
```

## Architecture Overview

### Layer Structure

The codebase follows a strict 4-layer architecture:

1. **Core Infrastructure** (`allocator.rs`, `panic_handler.rs`, `intrinsics.rs`)
   - Global allocator using UEFI AllocatePool/FreePool with arbitrary alignment (8-4096 bytes)
   - Panic handler with colored console output
   - Multi-architecture compiler intrinsics (x86_64, x86, aarch64)

2. **FFI Layer** (`ffi/`)
   - Raw C-style bindings to UEFI specification
   - All functions marked `unsafe`, exact struct layouts with `#[repr(C)]`
   - System Table, Boot Services Table, Runtime Services Table
   - Status codes, types, GUIDs

3. **Safe Wrapper Layer** (`boot_services/safe_wrappers.rs`, `runtime_services/safe_wrappers.rs`)
   - Transforms unsafe FFI to safe, idiomatic Rust
   - Result types instead of Status codes
   - RAII patterns for automatic resource cleanup (Events, Timers)
   - Null pointer handling with Option types

4. **High-Level Abstractions** (`protocols/`, `tables/`, utilities)
   - Type-safe, ergonomic APIs for protocols (Console, Storage, Network, Security, etc.)
   - ACPI and SMBIOS table parsing
   - String handling (UTF-8 ↔ UCS-2/UTF-16 conversion)
   - Logging framework, GUID management

### Key Architecture Patterns

**RAII for Resources**: All UEFI resources (Events, Timers, handles) are wrapped in structs with Drop implementations for automatic cleanup.

**FFI Safety Boundary**: All unsafe FFI calls are isolated in the FFI layer. Public APIs exposed to applications are safe unless explicitly marked unsafe with documented safety requirements.

**Error Handling**: Status codes converted to Result<T, Status>. Error propagation uses `?` operator.

**Memory Allocation**: Custom GlobalAlloc implementation handles alignment up to 4096 bytes by over-allocating and storing original pointer in header for later deallocation.

## Module Organization

### Core Modules
- `allocator.rs` - Global allocator implementation (CRITICAL: must be initialized before any allocations)
- `panic_handler.rs` - Panic handler with console output
- `intrinsics.rs` - Compiler intrinsics (memcpy, memset, 64-bit math for 32-bit)
- `system_table.rs` - System Table wrapper

### FFI Layer (`ffi/`)
- `types.rs` - UEFI primitive types (Uint8, Uint16, Handle, Boolean, etc.)
- `status.rs` - Status codes and Result type
- `table_header.rs` - Common table header structure
- `mod.rs` - System Table, Boot Services, Runtime Services definitions

### Services
- `boot_services/` - Memory, events, protocols, TPL management
  - `events.rs` - Event and Timer RAII wrappers
  - `tpl.rs` - Task Priority Level guards
  - `safe_wrappers.rs` - Safe Boot Services API
- `runtime_services/` - Time, variables, reset
  - `time.rs` - Time services
  - `variables.rs` - UEFI variable access
  - `safe_wrappers.rs` - Safe Runtime Services API

### Protocols (`protocols/`)
23 protocols implemented with comprehensive safe wrappers:
- Console I/O: `simple_text_input.rs`, `simple_text_output.rs`, `graphics_output.rs`
- **Storage** (with safe wrappers): `block_io.rs`, `simple_file_system.rs`, `storage.rs`
  - SCSI Pass Thru: `SafeScsiPassThru` with device iteration, command sending, reset operations
  - NVMe Pass Thru: `SafeNvmePassThru` with namespace iteration, Identify commands
  - Disk I/O/Disk I/O 2: `SafeDiskIo`, `SafeDiskIo2` for sync/async byte-level access
  - Partition Info: `SafePartitionInfo` with MBR/GPT helpers
  - SCSI Command Builders: `scsi_builder` module for INQUIRY, READ(10), WRITE(10)
- Network: `simple_network.rs`, `http.rs`, `tcp_udp.rs`, `ip.rs`, `pxe.rs`
- Hardware: `pci_io.rs`, `usb_io.rs`, `device_path.rs`
- System: `loaded_image.rs`, `firmware_management.rs`, `driver_binding.rs`, `mp_services.rs`
- **Security** (enhanced with safe wrappers): `security.rs`
  - Hash Protocol: `SafeHashProtocol` with SHA256/384/512 convenience methods
  - PKCS7 Verify: `SafePkcs7Verify` for signature verification
  - TPM 2.0: `SafeTpm2` with Startup, PCR Read, command submission
  - Secure Boot Helpers: PK/KEK/db/dbx/dbt access, Setup Mode detection, hash-in-database checking
- UI: `hii.rs`, `shell.rs`

### Firmware Tables (`tables/`)
- `acpi.rs` - Basic ACPI tables (RSDP, RSDT, XSDT, FADT, MADT)
- `acpi_advanced.rs` - Advanced ACPI (HPET, MCFG, BGRT, DMAR)
- `smbios.rs` - SMBIOS 2.x/3.0 parsing
- `configuration.rs` - Configuration table access

### Utilities
- `string.rs` - UTF-8 ↔ UCS-2 conversion, `ucs2!()` macro
- `guid.rs` - GUID operations
- `logger.rs` - 5-level logging framework
- `debug/serial.rs` - Serial debug output (COM1-4)
- `graphics/bmp.rs` - BMP graphics library

## Important Implementation Details

### Allocator Initialization
The allocator MUST be initialized before any heap allocations (Vec, String, Box, etc.):
```rust
unsafe {
    let bs = (*system_table).boot_services();
    allocator::init_allocator(bs);
}
```

### String Handling
UEFI uses UCS-2 (16-bit) strings, Rust uses UTF-8:
```rust
// Compile-time conversion
let hello = ucs2!("Hello");  // [u16; 6]

// Runtime conversion
use string::{to_ucs2, from_ucs2};
let ucs2_string = to_ucs2("Hello");
let rust_string = from_ucs2(&ucs2_vec);
```

### Protocol Location Pattern
```rust
// 1. Locate protocol (returns raw pointer)
let protocol_ptr = boot_services.locate_protocol(&PROTOCOL_GUID)?;

// 2. Cast to specific protocol type
let protocol = unsafe { &*(protocol_ptr as *const ProtocolStruct) };

// 3. Use safe wrapper if available
let safe_protocol = SafeProtocol::new(protocol)?;
```

### Event and Timer Pattern
```rust
use boot_services::events::{Event, Timer};

// Events automatically closed on drop
let event = Event::new(boot_services, EVT_NOTIFY_WAIT)?;
event.wait()?;
// Automatically calls CloseEvent when dropped

// Timers are events with timing
let timer = Timer::new(boot_services)?;
timer.set_timer_relative(10_000_000)?; // 1 second in 100ns units
```

### TPL Management
```rust
use boot_services::tpl::TplGuard;

// RAII guard automatically restores TPL
{
    let _guard = TplGuard::raise(boot_services, TPL_NOTIFY)?;
    // High-priority work here
} // TPL automatically restored
```

## Testing Strategy

Tests use mock UEFI environment to run natively without UEFI hardware:

- **Unit Tests** (`tests/`): Comprehensive protocol testing
  - `storage_tests.rs`: 600+ lines testing SCSI/NVMe/DiskIO protocols and command builders
  - `security_tests.rs`: 500+ lines testing Hash, PKCS7, TPM 2.0, Secure Boot
  - `protocol_tests.rs`, `boot_services_tests.rs`, `runtime_services_tests.rs`, etc.
- **Property Tests** (`tests/property_tests.rs`): 680+ lines verifying invariants
  - GUID uniqueness across all protocols
  - Structure size/alignment correctness
  - Encoding/decoding reversibility
  - Cross-protocol consistency checks
- **QEMU Integration** (`tests/qemu_*.rs`): Planned hardware-level automation

Mock environment is in `tests/mock_uefi.rs` and `tests/mock_environment.rs`.

**Total Test Count**: 1,400+ tests (unit + property + integration)
Current coverage estimate: ~70-75%, with comprehensive protocol coverage

See `TEST_PLAN.md` for further expansion.

## Common Development Patterns

### Adding a New Protocol

1. **FFI Definition** in `src/protocols/`:
   ```rust
   #[repr(C)]
   pub struct MyProtocol {
       pub revision: Uint64,
       pub my_function: extern "efiapi" fn(...) -> Status,
   }
   pub const MY_PROTOCOL_GUID: Guid = guid!("...");
   ```

2. **Safe Wrapper**:
   ```rust
   pub struct SafeMyProtocol<'a> {
       protocol: &'a MyProtocol,
   }
   impl SafeMyProtocol<'_> {
       pub fn do_thing(&self, args: T) -> Result<U, Status> {
           let status = unsafe { (self.protocol.my_function)(...) };
           if status != EFI_SUCCESS { return Err(status); }
           Ok(result)
       }
   }
   ```

3. **Tests** in `tests/protocol_tests.rs`:
   - Create mock protocol in `tests/mock_environment.rs`
   - Test success cases
   - Test error paths (invalid params, timeouts, etc.)
   - Test edge cases (null, overflow, etc.)

### Adding Safe Wrappers for Existing Protocols

Priority: Storage protocols (SCSI, NVMe, DiskIO) and Security protocols (PKCS7, TPM 2.0)

Follow the same pattern as existing safe wrappers in `protocols/` modules.

### Working with ACPI/SMBIOS Tables

```rust
use tables::acpi::AcpiTableFinder;
use tables::smbios::SmbiosIterator;

// Find ACPI table by signature
let fadt = AcpiTableFinder::find_table(system_table, b"FACP")?;

// Iterate SMBIOS entries
let smbios = SmbiosIterator::from_system_table(system_table)?;
for entry in smbios {
    // Process entry
}
```

## Recent Additions (2025)

### Storage Protocol Safe Wrappers (~800 lines)
- `SafeScsiPassThru`: Iterator-based device enumeration, command sending, reset operations
- `SafeNvmePassThru`: Namespace iteration, Identify Controller/Namespace commands
- `SafeDiskIo`/`SafeDiskIo2`: Synchronous and asynchronous byte-level disk access
- `SafePartitionInfo`: MBR/GPT information extraction with type-safe accessors
- `scsi_builder` module: Command builders for INQUIRY, READ(10), WRITE(10)

### Security Protocol Enhancements (~600 lines)
- `SafeHashProtocol`: SHA256/384/512 convenience methods
- `SafePkcs7Verify`: Buffer and signature verification
- `SafeTpm2`: TPM 2.0 command submission, Startup, PCR Read
- Enhanced Secure Boot helpers: Setup Mode, PK/KEK/db/dbx/dbt variables, hash verification

### Comprehensive Test Suite (~1,400 lines)
- Storage protocol tests: SCSI command builders, LBA encoding, partition structures
- Security protocol tests: Hash outputs, TPM 2.0 commands, signature lists
- Property-based tests: GUID uniqueness, reversibility, cross-protocol consistency

## Known Issues and Limitations

1. **Library builds successfully** on all architectures (x86_64, ARM64, i686)
2. **Example binaries** may need minor updates for new safe wrapper APIs
3. **Remaining ~10% work** focuses on:
   - QEMU test harness automation
   - Additional hardware-level integration tests
   - Minor test compilation fixes (imports)
   - Achieving 80%+ test coverage target

See `PROJECT_COMPLETION_PLAN.md` for detailed roadmap.

## License and Compatibility

- **License**: BSD-2-Clause-Patent (compatible with TianoCore EDK II)
- **UEFI Spec**: 2.10 compliance
- **Architectures**: x86_64 (primary), aarch64 (planned), i686 (planned)

## Important Notes for Development

- **Always use `cargo build --lib` first** to verify setup before building binaries
- **No_std compatibility**: Never add dependencies that require std
- **Safety boundary**: Keep all `unsafe` code in FFI layer, provide safe public APIs
- **RAII patterns**: All resources must have Drop implementations
- **Error handling**: Use Result types, never panic in library code (except panic_handler)
- **Testing**: Every new protocol needs unit tests with mocks
- **Documentation**: All public APIs need rustdoc comments with examples
- **Alignment**: Allocator supports up to 4096-byte alignment for DMA
- **Calling convention**: All UEFI function pointers use `extern "efiapi"`
