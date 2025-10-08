# UEFI Rust Integration - Build Guide

## Overview

This project implements BSD-2-Clause-Patent licensed Rust support for UEFI firmware, providing a `no_std` runtime environment that allows Rust code to run directly in UEFI without requiring a C runtime.

## License

All code is licensed under BSD-2-Clause-Patent to ensure compatibility with TianoCore EDK II.

## Prerequisites

1. **Rust Nightly Toolchain**
   ```bash
   rustup toolchain install nightly
   rustup default nightly
   ```

2. **x86_64-unknown-uefi Target**
   The target is built automatically using `build-std` feature

## Building

### Build the UEFI Application

```bash
cargo +nightly build --target x86_64-unknown-uefi -Zbuild-std=core,compiler_builtins,alloc -Zbuild-std-features=compiler-builtins-mem
```

Or simply:
```bash
cargo build
```

The configuration in `.cargo/config.toml` automatically sets the target and build-std flags.

### Release Build

```bash
cargo build --release
```

The binary will be located at:
- Debug: `target/x86_64-unknown-uefi/debug/uefi-app.efi`
- Release: `target/x86_64-unknown-uefi/release/uefi-app.efi`

## Running

### Using QEMU with OVMF

1. **Install QEMU and OVMF**
   - Windows: Download from qemu.org
   - Linux: `sudo apt install qemu-system-x86 ovmf`
   - macOS: `brew install qemu`

2. **Create a UEFI disk image**
   ```bash
   # Create a FAT32 disk image
   dd if=/dev/zero of=disk.img bs=1M count=64
   mkfs.fat -F 32 disk.img

   # Mount and copy the EFI binary
   mkdir -p mnt
   sudo mount disk.img mnt
   sudo mkdir -p mnt/EFI/BOOT
   sudo cp target/x86_64-unknown-uefi/debug/uefi-app.efi mnt/EFI/BOOT/BOOTX64.EFI
   sudo umount mnt
   ```

3. **Run with QEMU**
   ```bash
   qemu-system-x86_64 \
     -bios /usr/share/ovmf/OVMF.fd \
     -drive format=raw,file=disk.img \
     -net none
   ```

### Using Real Hardware or Virtual Machine

1. Copy the `.efi` file to a USB drive formatted as FAT32
2. Place it in `/EFI/BOOT/BOOTX64.EFI` on the USB drive
3. Boot from the USB drive in UEFI mode

## Project Structure

```
uefi-rust-integration/
├── src/
│   ├── lib.rs                      # Library root with panic handler
│   ├── ffi/                        # Raw UEFI FFI bindings
│   │   ├── mod.rs                  # Core FFI types
│   │   ├── types.rs                # Basic UEFI types
│   │   ├── status.rs               # EFI_STATUS codes
│   │   └── table_header.rs         # EFI_TABLE_HEADER
│   ├── boot_services/              # Boot Services
│   │   ├── mod.rs                  # Boot Services table
│   │   └── safe_wrappers.rs        # Safe Rust wrappers
│   ├── runtime_services/           # Runtime Services
│   │   ├── mod.rs                  # Runtime Services table
│   │   └── safe_wrappers.rs        # Safe Rust wrappers
│   ├── system_table.rs             # EFI System Table
│   ├── allocator.rs                # Global allocator using AllocatePool
│   ├── protocols/                  # UEFI protocols
│   └── bin/
│       └── main.rs                 # Application entry point
├── .cargo/
│   └── config.toml                 # Cargo configuration
├── Cargo.toml                      # Package manifest
└── BUILD.md                        # This file
```

## Key Components

### 1. No_std Runtime
- Custom panic handler
- Language items (eh_personality)
- Compiler built-ins (memcpy, memset, memcmp)

### 2. Global Allocator
- Uses UEFI's `AllocatePool` and `FreePool` services
- Enables use of `alloc` collections (Vec, String, etc.)

### 3. FFI Bindings
- Raw C-compatible types matching UEFI specification
- Boot Services and Runtime Services tables
- System Table and Protocol definitions

### 4. Safe Abstractions
- Rust-friendly wrappers around unsafe FFI calls
- Result-based error handling
- Type-safe protocol access

## Integration with EDK II

To integrate with EDK II build system:

1. **Create .inf file** for the Rust module
2. **Generate object files** from Rust using:
   ```bash
   cargo rustc --release -- --emit obj
   ```
3. **Link with EDK II** using the EDK II build tools
4. **Configure BaseTools** to recognize Rust source files

This is still experimental and requires further development of the EDK II integration layer.

## Current Status

✅ Completed:
- No_std Rust configuration
- UEFI FFI bindings (types, Boot Services, Runtime Services)
- Panic handler and language items
- Global allocator using UEFI AllocatePool
- UEFI entry point
- Safe Rust abstractions
- Example UEFI application

⚠️ In Progress:
- EDK II BaseTools integration
- Additional protocol implementations
- Enhanced error handling
- Comprehensive testing framework

## Contributing

When contributing:
1. Ensure all code is BSD-2-Clause-Patent licensed
2. Follow UEFI specification 2.10
3. Maintain `no_std` compatibility
4. Add tests where possible
5. Document all public APIs

## Resources

- [UEFI Specification 2.10](https://uefi.org/specifications)
- [TianoCore EDK II](https://github.com/tianocore/edk2)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [EDK II Rust Task](https://github.com/tianocore/tianocore.github.io/wiki/Tasks-Add-Rust-Support-to-EDK-II)
