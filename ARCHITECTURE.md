# UEFI Rust Implementation - Architecture

This document describes the architectural design of the UEFI Rust implementation.

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      UEFI Application                            │
│                    (Your Firmware Code)                          │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         │ Safe Rust API
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                  UEFI Rust Library                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │           High-Level Safe Abstractions                   │  │
│  │  • Protocols (Graphics, Network, Storage, etc.)          │  │
│  │  • Services (Boot, Runtime, Events, Timers)              │  │
│  │  • Utilities (String, GUID, Logging)                     │  │
│  │  • Tables (ACPI, SMBIOS)                                 │  │
│  └────────────────────┬─────────────────────────────────────┘  │
│                       │                                          │
│  ┌────────────────────▼─────────────────────────────────────┐  │
│  │          Safe Wrapper Layer                              │  │
│  │  • Result types                                          │  │
│  │  • RAII patterns                                         │  │
│  │  • Error conversion                                      │  │
│  │  • Null pointer handling                                 │  │
│  └────────────────────┬─────────────────────────────────────┘  │
│                       │                                          │
│  ┌────────────────────▼─────────────────────────────────────┐  │
│  │          FFI Layer (unsafe)                              │  │
│  │  • Raw C-style structures                                │  │
│  │  • Function pointers                                     │  │
│  │  • extern "efiapi" declarations                          │  │
│  └────────────────────┬─────────────────────────────────────┘  │
│                       │                                          │
│  ┌────────────────────▼─────────────────────────────────────┐  │
│  │        Core Infrastructure                               │  │
│  │  • Global Allocator                                      │  │
│  │  • Panic Handler                                         │  │
│  │  • Compiler Intrinsics                                   │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         │ UEFI C ABI
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                   UEFI Firmware                                  │
│           (TianoCore EDK II / Vendor Firmware)                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Layer Descriptions

### 1. Core Infrastructure Layer

The foundation layer that enables Rust to work in a no_std UEFI environment.

**Components:**
- **Global Allocator** (`allocator.rs`)
  - Implements `GlobalAlloc` trait
  - Uses UEFI AllocatePool/FreePool
  - Supports arbitrary alignment (8-4096 bytes)
  - Tracks allocations with headers

- **Panic Handler** (`panic_handler.rs`)
  - Captures panic location and message
  - Outputs to UEFI console with colors
  - Multi-architecture halt (x86_64, x86, aarch64)
  - Graceful error display

- **Compiler Intrinsics** (`intrinsics.rs`)
  - memcpy, memset, memcmp implementations
  - 64-bit math for 32-bit platforms
  - Floating-point operations
  - Architecture-specific optimizations

**Responsibilities:**
- Enable Rust standard library features (alloc, collections)
- Provide runtime support
- Handle low-level operations

---

### 2. FFI (Foreign Function Interface) Layer

Raw bindings to UEFI C APIs following the UEFI specification.

**Components:**
- **Types** (`ffi/types.rs`)
  ```
  • Uint8, Uint16, Uint32, Uint64, Uintn
  • Boolean, Char16, Handle, Event
  • Status codes, GUIDs
  ```

- **System Table** (`ffi/mod.rs`)
  ```
  ┌──────────────────┐
  │  System Table    │
  ├──────────────────┤
  │ • Header         │
  │ • FirmwareVendor │
  │ • ConIn          │───► Simple Text Input Protocol
  │ • ConOut         │───► Simple Text Output Protocol
  │ • StdErr         │───► Simple Text Output Protocol
  │ • RuntimeServices│───► Runtime Services Table
  │ • BootServices   │───► Boot Services Table
  │ • ConfigTable    │───► Configuration Tables (ACPI, SMBIOS)
  └──────────────────┘
  ```

- **Boot Services Table** (`boot_services/mod.rs`)
  ```
  • Memory Services (AllocatePool, FreePool, GetMemoryMap)
  • Event Services (CreateEvent, WaitForEvent, SignalEvent)
  • Task Priority (RaiseTPL, RestoreTPL)
  • Protocol Services (LocateProtocol, HandleProtocol)
  • Image Services (LoadImage, StartImage, Exit)
  ```

- **Runtime Services Table** (`runtime_services/mod.rs`)
  ```
  • Time Services (GetTime, SetTime, GetWakeupTime)
  • Variable Services (GetVariable, SetVariable, NextVariableName)
  • Virtual Memory (ConvertPointer, SetVirtualAddressMap)
  • Reset Services (ResetSystem)
  • Capsule Services (UpdateCapsule, QueryCapsuleCapabilities)
  ```

**Characteristics:**
- All functions marked `unsafe`
- Direct C struct layouts with `#[repr(C)]`
- No Rust safety guarantees
- Exact match to UEFI specification

---

### 3. Safe Wrapper Layer

Transforms unsafe FFI into safe, idiomatic Rust.

**Patterns:**

**Result Types:**
```rust
// FFI: Returns Status code
unsafe extern "efiapi" fn(args) -> Status

// Wrapper: Returns Result
fn safe_function(args) -> Result<T, Status>
```

**RAII (Resource Acquisition Is Initialization):**
```rust
pub struct Event {
    handle: EventHandle,
    boot_services: &'static BootServices,
}

impl Drop for Event {
    fn drop(&mut self) {
        // Automatically close event
        unsafe {
            (self.boot_services.close_event)(self.handle);
        }
    }
}
```

**Null Pointer Handling:**
```rust
// FFI: Accepts null pointers
unsafe fn ffi_call(ptr: *const T);

// Wrapper: Uses Option
fn safe_call(value: Option<&T>);
```

---

### 4. High-Level Abstractions Layer

Provides convenient, type-safe APIs for common operations.

```
┌─────────────────────────────────────────────────────────────┐
│                    High-Level Layer                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐  │
│  │   Protocols   │  │   Services    │  │   Utilities   │  │
│  ├───────────────┤  ├───────────────┤  ├───────────────┤  │
│  │ • Graphics    │  │ • Variables   │  │ • String      │  │
│  │ • Network     │  │ • Events      │  │ • GUID        │  │
│  │ • Storage     │  │ • Timers      │  │ • Logging     │  │
│  │ • Security    │  │ • TPL         │  │ • Debug       │  │
│  │ • USB/PCI     │  │ • Memory      │  └───────────────┘  │
│  │ • HII         │  └───────────────┘                      │
│  │ • Shell       │                                          │
│  └───────────────┘                                          │
│                                                               │
│  ┌───────────────┐  ┌───────────────┐                      │
│  │    Tables     │  │   Graphics    │                      │
│  ├───────────────┤  ├───────────────┤                      │
│  │ • ACPI        │  │ • BMP         │                      │
│  │ • SMBIOS      │  │ • GOP         │                      │
│  │ • Config      │  │ • Framebuffer │                      │
│  └───────────────┘  └───────────────┘                      │
└─────────────────────────────────────────────────────────────┘
```

---

## Protocol Architecture

Protocols are the primary mechanism for device and service interaction in UEFI.

```
┌──────────────────────────────────────────────────────────┐
│                    Protocol Locator                       │
│  BootServices.LocateProtocol() / HandleProtocol()        │
└────────────────────┬─────────────────────────────────────┘
                     │
      ┌──────────────┼──────────────┬──────────────┐
      │              │               │              │
      ▼              ▼               ▼              ▼
┌──────────┐  ┌──────────┐   ┌──────────┐   ┌──────────┐
│ Console  │  │ Network  │   │ Storage  │   │ Security │
│ Protocols│  │ Protocols│   │ Protocols│   │ Protocols│
├──────────┤  ├──────────┤   ├──────────┤   ├──────────┤
│• TextIn  │  │• SNP     │   │• BlockIO │   │• Hash    │
│• TextOut │  │• IP4/6   │   │• FileIO  │   │• PKCS7   │
│• GOP     │  │• TCP/UDP │   │• SCSI    │   │• SecBoot │
└──────────┘  │• HTTP    │   │• NVMe    │   └──────────┘
              │• PXE     │   │• DiskIO  │
              │• DHCP    │   └──────────┘
              │• DNS     │
              └──────────┘
```

**Protocol Location Flow:**
1. Application calls `LocateProtocol(&GUID)`
2. UEFI firmware searches protocol database
3. Returns pointer to protocol interface
4. Application casts to specific protocol structure
5. Calls protocol functions through function pointers

**Our Safe Wrapper Approach:**
```rust
// 1. Locate protocol (unsafe FFI)
let protocol_ptr = unsafe {
    let mut ptr: *mut Protocol = core::ptr::null_mut();
    (boot_services.locate_protocol)(
        &PROTOCOL_GUID,
        core::ptr::null_mut(),
        &mut ptr as *mut _ as *mut _,
    )?;
    ptr
};

// 2. Create safe wrapper
let safe_protocol = SafeProtocol::new(unsafe { &*protocol_ptr })?;

// 3. Use safe API
safe_protocol.do_something()?;
```

---

## Memory Management

```
┌──────────────────────────────────────────────────────────────┐
│                    Application Code                           │
│  let v = Vec::new();  // Allocates from Rust allocator       │
└────────────────────────┬─────────────────────────────────────┘
                         │ alloc::alloc::alloc()
                         ▼
┌──────────────────────────────────────────────────────────────┐
│                  GlobalAlloc Trait                            │
│  fn alloc(&self, layout: Layout) -> *mut u8                  │
└────────────────────────┬─────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────┐
│              UefiAllocator Implementation                     │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ if layout.align() <= 8 {                                │ │
│  │     // Standard allocation                              │ │
│  │     AllocatePool(size)                                  │ │
│  │ } else {                                                 │ │
│  │     // Over-allocate for alignment                      │ │
│  │     let alloc_size = size + align + HEADER_SIZE;        │ │
│  │     let ptr = AllocatePool(alloc_size);                 │ │
│  │     let aligned = align_up(ptr + HEADER_SIZE, align);   │ │
│  │     // Store original pointer in header                 │ │
│  │     return aligned;                                      │ │
│  │ }                                                         │ │
│  └────────────────────────────────────────────────────────┘ │
└────────────────────────┬─────────────────────────────────────┘
                         │ UEFI C ABI
                         ▼
┌──────────────────────────────────────────────────────────────┐
│        UEFI AllocatePool / FreePool                          │
│  Firmware memory manager                                     │
└──────────────────────────────────────────────────────────────┘
```

**Alignment Header Structure:**
```
For allocation with align > 8:

Original allocation:
┌──────┬────────────┬─────────────────────────┬────────┐
│ gap  │ Header (8) │  Aligned user data      │  gap   │
│      │   [ptr]    │                         │        │
└──────┴────────────┴─────────────────────────┴────────┘
       ▲            ▲
       │            └─ Returned to user (aligned)
       └─ Stored in header for later free
```

---

## Event and Timer System

```
┌──────────────────────────────────────────────────────────────┐
│                     Event System                              │
├──────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌────────────┐      ┌────────────┐      ┌────────────┐     │
│  │  Event     │      │  Timer     │      │ TPL Guard  │     │
│  ├────────────┤      ├────────────┤      ├────────────┤     │
│  │ handle     │      │ event      │      │ old_tpl    │     │
│  │ bs*        │      │ bs*        │      │ bs*        │     │
│  └────────────┘      └────────────┘      └────────────┘     │
│       │                   │                    │              │
│       │ Drop              │ Drop               │ Drop         │
│       ▼                   ▼                    ▼              │
│  CloseEvent         CloseEvent          RestoreTPL           │
│                                                                │
└──────────────────────────────────────────────────────────────┘

Event Creation Flow:
1. CreateEvent() → Returns Event handle
2. Wrap in RAII Event struct
3. Use event (WaitForEvent, CheckEvent)
4. Drop automatically calls CloseEvent

Timer Creation Flow:
1. CreateEvent(EVT_TIMER) → Returns Event handle
2. SetTimer(handle, type, trigger_time)
3. Wrap in Timer struct
4. Drop automatically calls CloseEvent
```

---

## String Handling

UEFI uses UCS-2 (16-bit) strings, Rust uses UTF-8.

```
┌──────────────────────────────────────────────────────────────┐
│                    String Conversion                          │
├──────────────────────────────────────────────────────────────┤
│                                                                │
│  Rust UTF-8 (&str)                                            │
│         │                                                      │
│         │ to_ucs2()                                           │
│         ▼                                                      │
│  Vec<u16> (UCS-2)                                             │
│         │                                                      │
│         │ as_ptr()                                            │
│         ▼                                                      │
│  *const Char16  ────────► UEFI API                           │
│         ▲                                                      │
│         │ from_ucs2()                                         │
│         │                                                      │
│  String (UTF-8) ◄──────── UEFI API returns *const Char16     │
│                                                                │
└──────────────────────────────────────────────────────────────┘

Macro for compile-time conversion:
ucs2!("Hello") → [u16; 6] = [72, 101, 108, 108, 111, 0]
```

---

## Firmware Table Access

```
┌──────────────────────────────────────────────────────────────┐
│                 Configuration Table                           │
│  SystemTable.ConfigurationTable[n]                           │
│                                                                │
│  ┌──────────────┬──────────────────────────────┐            │
│  │ GUID         │ VendorTable Pointer          │            │
│  ├──────────────┼──────────────────────────────┤            │
│  │ ACPI_20_GUID │ → RSDP Pointer              │            │
│  │ SMBIOS_3_0   │ → SMBIOS Entry Point        │            │
│  │ ...          │ → ...                        │            │
│  └──────────────┴──────────────────────────────┘            │
└────────────────────────┬─────────────────────────────────────┘
                         │
         ┌───────────────┴───────────────┐
         │                               │
         ▼                               ▼
┌─────────────────┐            ┌─────────────────┐
│  ACPI Tables    │            │  SMBIOS Tables  │
├─────────────────┤            ├─────────────────┤
│ RSDP            │            │ Entry Point     │
│   │             │            │   │             │
│   ├─► RSDT/XSDT │            │   ├─► Type 0    │
│       │         │            │   ├─► Type 1    │
│       ├─► FADT  │            │   ├─► Type 2    │
│       ├─► MADT  │            │   ├─► Type 4    │
│       ├─► HPET  │            │   └─► ...       │
│       ├─► MCFG  │            │                  │
│       └─► ...   │            └─────────────────┘
└─────────────────┘

Access Pattern:
1. Find configuration table by GUID
2. Cast vendor table pointer to specific type
3. Validate checksums
4. Parse table entries
5. Iterate subtables
```

---

## Testing Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      Test Strategy                            │
├──────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Unit Tests (Native)                          │   │
│  │  • Test individual functions                         │   │
│  │  • Mock UEFI services                                │   │
│  │  • Fast feedback loop                                │   │
│  │  • High coverage                                     │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │       Integration Tests (QEMU)                       │   │
│  │  • Boot actual UEFI application                      │   │
│  │  • Test protocol interactions                        │   │
│  │  • Validate on OVMF firmware                         │   │
│  │  • Automated via test harness                        │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │      Property Tests (Native)                         │   │
│  │  • Test invariants                                   │   │
│  │  • Random input generation                           │   │
│  │  • Edge case discovery                               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                                │
└──────────────────────────────────────────────────────────────┘

Mock Environment:
┌────────────────────────────────────────────┐
│ Mock BootServices                           │
│  • Fake AllocatePool (uses std allocator)  │
│  • Fake protocol database                  │
│  • Fake event system                       │
│  • Validates parameters                    │
└────────────────────────────────────────────┘
```

---

## Build Process

```
┌──────────────────────────────────────────────────────────────┐
│                     Build Flow                                │
├──────────────────────────────────────────────────────────────┤
│                                                                │
│  Rust Source Files                                            │
│         │                                                      │
│         │ cargo build --target x86_64-unknown-uefi           │
│         ▼                                                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  rustc with -Zbuild-std                              │   │
│  │  • Compile core, alloc, compiler_builtins           │   │
│  │  • no_std mode                                       │   │
│  │  • UEFI calling convention                           │   │
│  └────────────────┬─────────────────────────────────────┘   │
│                   │                                            │
│                   ▼                                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  .rlib files                                         │   │
│  └────────────────┬─────────────────────────────────────┘   │
│                   │                                            │
│                   │ Link                                       │
│                   ▼                                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  .efi file (PE/COFF format)                          │   │
│  │  • UEFI subsystem                                    │   │
│  │  • efi_main entry point                              │   │
│  │  • Relocatable code                                  │   │
│  └──────────────────────────────────────────────────────┘   │
│                   │                                            │
│                   │ Deploy                                     │
│                   ▼                                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  UEFI System Partition (ESP)                         │   │
│  │  /EFI/BOOT/BOOTX64.EFI                              │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                                │
└──────────────────────────────────────────────────────────────┘
```

---

## Design Principles

### 1. Safety First
- Minimize unsafe code
- Encapsulate all unsafe operations
- Provide safe public APIs
- Use Rust type system for correctness

### 2. Zero-Cost Abstractions
- Wrappers compile to same code as raw FFI
- No runtime overhead
- Inline small functions
- Optimize for performance

### 3. Idiomatic Rust
- Use Result for errors
- RAII for resource management
- Traits for common interfaces
- Iterator patterns where applicable

### 4. UEFI Compliance
- Follow UEFI 2.10 specification
- Match C struct layouts exactly
- Respect calling conventions
- Handle all error cases

### 5. Testability
- Mock all UEFI interactions
- Unit test without hardware
- Integration test with QEMU
- Property test for invariants

---

## Key Architectural Decisions

### Why Not Use r-efi?
**Decision:** Implement custom FFI bindings
**Rationale:**
- Full control over API design
- Tailored error handling
- Integrated safe wrappers
- BSD-2-Clause-Patent license compatibility

### Why Global Allocator?
**Decision:** Implement custom allocator instead of using uefi-rs
**Rationale:**
- Support arbitrary alignment (required for DMA)
- Better error handling (no panic on OOM)
- Integrated with Rust's alloc crate
- Full control over allocation strategy

### Why RAII Wrappers?
**Decision:** Wrap all resources (Events, Timers) in RAII types
**Rationale:**
- Automatic cleanup prevents leaks
- Compile-time resource management
- Idiomatic Rust patterns
- Prevents use-after-free

### Why Separate FFI Layer?
**Decision:** Clear separation between FFI and safe wrappers
**Rationale:**
- Audit unsafe code easily
- Multiple safe wrapper strategies
- Clear API boundaries
- Easier to maintain

---

## Performance Considerations

### Memory Allocation
- **Pool Type:** EfiLoaderData for most allocations
- **Alignment:** Over-allocate only when needed (align > 8)
- **Caching:** UEFI firmware handles pool caching

### Protocol Access
- **Location:** Cache protocol pointers when possible
- **Lookup:** Use HandleProtocol when handle is known
- **Multiple:** Use LocateHandleBuffer for bulk operations

### String Conversion
- **UTF-8 → UCS-2:** One allocation, single pass
- **UCS-2 → UTF-8:** Zero-copy when possible
- **Stack:** Use stack buffer for short strings

### Event Handling
- **TPL:** Minimize time at raised TPL
- **Polling:** Use WaitForEvent instead of polling
- **Batching:** Batch multiple operations when possible

---

## Future Architectural Enhancements

### Considered But Not Yet Implemented

1. **Async/Await Support**
   - UEFI has no native async model
   - Could build on top of events
   - Significant complexity

2. **Plugin System**
   - Dynamic protocol registration
   - Runtime feature loading
   - Needs careful design

3. **DMA Helpers**
   - Safe DMA buffer management
   - Memory barrier abstractions
   - Cache coherency helpers

4. **Advanced Error Types**
   - Rich error context
   - Error chains
   - Structured errors

---

## Conclusion

This architecture provides a robust, safe, and performant foundation for UEFI firmware development in Rust. The layered design separates concerns, the safety wrappers prevent common errors, and the idiomatic Rust APIs make firmware development more productive.

The architecture is designed to be:
- **Extensible:** New protocols easy to add
- **Maintainable:** Clear separation of concerns
- **Testable:** Mock environment for rapid iteration
- **Safe:** Minimal unsafe code, maximum compiler checking
- **Performant:** Zero-cost abstractions, no runtime overhead
