# UEFI Rust Implementation - Test Plan

**Current Coverage:** ~60%
**Target Coverage:** 80%+
**Focus:** Critical paths, error handling, edge cases

---

## Test Strategy Overview

Our testing approach is multi-layered to ensure reliability without requiring UEFI hardware:

```
┌─────────────────────────────────────────────────────────────┐
│                     Test Pyramid                             │
│                                                               │
│                      ┌──────────┐                            │
│                      │  Manual  │                            │
│                      │ Hardware │  (Future)                  │
│                      └────┬─────┘                            │
│                 ┌─────────┴─────────┐                        │
│                 │  QEMU Integration │  (10% of tests)        │
│                 └─────────┬─────────┘                        │
│            ┌──────────────┴──────────────┐                   │
│            │   Property-Based Tests      │  (15% of tests)   │
│            └──────────────┬──────────────┘                   │
│       ┌───────────────────┴───────────────────┐              │
│       │         Unit Tests                    │  (75% of tests) │
│       └───────────────────────────────────────┘              │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Storage Protocol Tests

### Test File: `tests/storage_protocol_tests.rs`
**New File:** ~500 lines
**Coverage Target:** 85%+

#### 1.1 SCSI Pass Thru Tests

```rust
#[test]
fn test_scsi_passthru_inquiry() {
    // Test standard INQUIRY command
    let mock_scsi = create_mock_scsi_protocol();
    let mut inquiry_data = [0u8; 36];
    let result = scsi_passthru.inquiry(target, lun, &mut inquiry_data);
    assert!(result.is_ok());
    assert_eq!(inquiry_data[0] & 0x1F, 0x00); // Direct-access device
}

#[test]
fn test_scsi_passthru_timeout() {
    // Test timeout handling
    let mock_scsi = create_mock_scsi_protocol_with_timeout();
    let result = scsi_passthru.read10(target, lun, lba, &mut buffer);
    assert_eq!(result.unwrap_err(), EFI_TIMEOUT);
}

#[test]
fn test_scsi_device_enumeration() {
    // Test get_next_device iterator
    let mock_scsi = create_mock_scsi_with_devices(3);
    let devices: Vec<_> = scsi_passthru.devices().collect();
    assert_eq!(devices.len(), 3);
}

#[test]
fn test_scsi_invalid_target() {
    // Test invalid target handling
    let result = scsi_passthru.send_command(999, 0, &command);
    assert_eq!(result.unwrap_err(), EFI_INVALID_PARAMETER);
}
```

**Test Cases:** 20+ tests
- Command execution (INQUIRY, READ, WRITE)
- Error handling (timeout, device errors)
- Device enumeration
- Invalid parameters
- Buffer overflow protection

---

#### 1.2 NVMe Pass Thru Tests

```rust
#[test]
fn test_nvme_identify_controller() {
    let mock_nvme = create_mock_nvme_protocol();
    let mut identify_data = [0u8; 4096];
    let result = nvme.identify_controller(&mut identify_data);
    assert!(result.is_ok());
}

#[test]
fn test_nvme_namespace_enumeration() {
    let mock_nvme = create_mock_nvme_with_namespaces(&[1, 2, 3]);
    let namespaces: Vec<_> = nvme.namespaces().collect();
    assert_eq!(namespaces.len(), 3);
}

#[test]
fn test_nvme_admin_command() {
    let mock_nvme = create_mock_nvme_protocol();
    let result = nvme.admin_command(&command);
    assert!(result.is_ok());
}

#[test]
fn test_nvme_io_command_timeout() {
    let mock_nvme = create_mock_nvme_with_timeout();
    let result = nvme.read(namespace_id, lba, &mut buffer);
    assert_eq!(result.unwrap_err(), EFI_TIMEOUT);
}
```

**Test Cases:** 15+ tests
- Identify commands
- Namespace management
- I/O commands
- Admin commands
- Timeout handling

---

#### 1.3 Disk I/O Tests

```rust
#[test]
fn test_disk_io_read_aligned() {
    let mock_disk = create_mock_disk_io();
    let mut buffer = vec![0u8; 512];
    let result = disk_io.read_disk(media_id, 0, &mut buffer);
    assert!(result.is_ok());
}

#[test]
fn test_disk_io_read_unaligned() {
    // Test reading at byte offset (not sector-aligned)
    let mock_disk = create_mock_disk_io();
    let mut buffer = vec![0u8; 100];
    let result = disk_io.read_disk(media_id, 256, &mut buffer);
    assert!(result.is_ok());
}

#[test]
fn test_disk_io_write() {
    let mock_disk = create_mock_disk_io();
    let data = vec![0xAA; 512];
    let result = disk_io.write_disk(media_id, 0, &data);
    assert!(result.is_ok());
}

#[test]
fn test_disk_io_invalid_media_id() {
    let result = disk_io.read_disk(999, 0, &mut buffer);
    assert_eq!(result.unwrap_err(), EFI_INVALID_PARAMETER);
}
```

**Test Cases:** 12+ tests
- Aligned reads/writes
- Unaligned operations
- Media ID validation
- Read-only media protection
- Large transfers

---

#### 1.4 Partition Info Tests

```rust
#[test]
fn test_partition_info_gpt() {
    let mock_partition = create_mock_partition_info(PartitionType::Gpt);
    let info = partition_info.get_info();
    assert_eq!(info.partition_type, PartitionType::Gpt);
    assert!(info.system_partition);
}

#[test]
fn test_partition_info_mbr() {
    let mock_partition = create_mock_partition_info(PartitionType::Mbr);
    let info = partition_info.get_info();
    assert_eq!(info.partition_type, PartitionType::Mbr);
}

#[test]
fn test_partition_guid_parsing() {
    let mock_partition = create_mock_partition_with_guid();
    let guid = partition_info.partition_guid();
    assert!(!guid.is_null());
}
```

**Test Cases:** 10+ tests
- GPT partitions
- MBR partitions
- GUID parsing
- System partition detection
- Invalid partition types

---

## Phase 2: Security Protocol Tests

### Test File: `tests/security_protocol_tests.rs`
**New File:** ~600 lines
**Coverage Target:** 90%+

#### 2.1 PKCS7 Verification Tests

```rust
#[test]
fn test_pkcs7_verify_valid_signature() {
    let mock_pkcs7 = create_mock_pkcs7_protocol();
    let signature = include_bytes!("test_data/signature.p7b");
    let data = b"Test data";
    let cert = include_bytes!("test_data/cert.der");

    let result = pkcs7.verify_signature(signature, cert, data);
    assert!(result.is_ok());
}

#[test]
fn test_pkcs7_verify_invalid_signature() {
    let mock_pkcs7 = create_mock_pkcs7_protocol();
    let signature = include_bytes!("test_data/invalid_signature.p7b");
    let data = b"Test data";
    let cert = include_bytes!("test_data/cert.der");

    let result = pkcs7.verify_signature(signature, cert, data);
    assert_eq!(result.unwrap_err(), EFI_SECURITY_VIOLATION);
}

#[test]
fn test_pkcs7_certificate_chain() {
    // Test certificate chain validation
    let mock_pkcs7 = create_mock_pkcs7_with_chain();
    let result = pkcs7.verify_with_chain(signature, certs, data);
    assert!(result.is_ok());
}

#[test]
fn test_pkcs7_expired_certificate() {
    let mock_pkcs7 = create_mock_pkcs7_with_expired_cert();
    let result = pkcs7.verify_signature(signature, expired_cert, data);
    assert_eq!(result.unwrap_err(), EFI_SECURITY_VIOLATION);
}
```

**Test Cases:** 25+ tests
- Valid signatures
- Invalid signatures
- Detached signatures
- Embedded signatures
- Certificate chains
- Expired certificates
- Revoked certificates
- Malformed PKCS7 data

**Test Data Required:**
- Generate test certificates with OpenSSL
- Create valid and invalid signatures
- Include in `tests/test_data/` directory

---

#### 2.2 Secure Boot Tests

```rust
#[test]
fn test_secure_boot_enabled() {
    let mock_vars = create_mock_variables_with_secure_boot();
    assert!(secure_boot::is_secure_boot_enabled(&mock_vars));
}

#[test]
fn test_secure_boot_disabled() {
    let mock_vars = create_mock_variables_without_secure_boot();
    assert!(!secure_boot::is_secure_boot_enabled(&mock_vars));
}

#[test]
fn test_read_platform_key() {
    let mock_vars = create_mock_variables();
    let mut buffer = vec![0u8; 1024];
    let pk = secure_boot::get_platform_key(&mock_vars, &mut buffer);
    assert!(pk.is_ok());
}

#[test]
fn test_read_signature_database() {
    let mock_vars = create_mock_variables();
    let db = secure_boot::get_signature_database(&mock_vars);
    assert!(db.is_ok());
    assert!(!db.unwrap().is_empty());
}

#[test]
fn test_check_signature_in_db() {
    let mock_vars = create_mock_variables_with_known_signature();
    let hash = [0xAA; 32]; // SHA256 hash
    let result = secure_boot::is_hash_in_db(&mock_vars, &hash);
    assert!(result);
}
```

**Test Cases:** 20+ tests
- Secure Boot status
- PK read/write
- KEK read/write
- db/dbx read/write
- Signature checking
- Variable authentication
- Setup mode detection

---

#### 2.3 TPM 2.0 Tests

```rust
#[test]
fn test_tpm_startup() {
    let mock_tpm = create_mock_tpm_protocol();
    let result = tpm.startup(TPM_SU_CLEAR);
    assert!(result.is_ok());
}

#[test]
fn test_tpm_get_capability() {
    let mock_tpm = create_mock_tpm_protocol();
    let caps = tpm.get_capability(TPM_CAP_TPM_PROPERTIES, 0x100);
    assert!(caps.is_ok());
}

#[test]
fn test_tpm_pcr_read() {
    let mock_tpm = create_mock_tpm_with_pcrs();
    let pcr_value = tpm.pcr_read(0); // PCR 0
    assert!(pcr_value.is_ok());
    assert_eq!(pcr_value.unwrap().len(), 32); // SHA256
}

#[test]
fn test_tpm_pcr_extend() {
    let mock_tpm = create_mock_tpm_protocol();
    let data = [0xBB; 32];
    let result = tpm.pcr_extend(7, &data); // PCR 7
    assert!(result.is_ok());
}

#[test]
fn test_tpm_not_present() {
    let mock_tpm = create_mock_tpm_absent();
    let result = tpm.startup(TPM_SU_CLEAR);
    assert_eq!(result.unwrap_err(), EFI_UNSUPPORTED);
}
```

**Test Cases:** 15+ tests
- TPM startup
- Capability queries
- PCR read/extend
- Random number generation
- Absence handling
- Command timeouts

---

## Phase 3: QEMU Integration Tests

### Test Harness: `tests/qemu_runner.rs`
**Enhancement:** ~300 lines

```rust
pub struct QemuTestHarness {
    qemu_binary: PathBuf,
    ovmf_firmware: PathBuf,
    test_disk: TempFile,
}

impl QemuTestHarness {
    pub fn new() -> Result<Self> {
        // Download OVMF if not present
        let ovmf = Self::ensure_ovmf()?;

        // Create virtual disk
        let disk = Self::create_test_disk()?;

        Ok(Self {
            qemu_binary: Self::find_qemu()?,
            ovmf_firmware: ovmf,
            test_disk: disk,
        })
    }

    pub fn run_test(&self, efi_app: &Path) -> TestResult {
        // Copy EFI app to ESP
        self.deploy_to_esp(efi_app)?;

        // Start QEMU with serial output
        let mut qemu = Command::new(&self.qemu_binary)
            .arg("-bios").arg(&self.ovmf_firmware)
            .arg("-drive").arg(format!("file={},format=raw",
                self.test_disk.path().display()))
            .arg("-serial").arg("stdio")
            .arg("-nographic")
            .arg("-net").arg("none")
            .spawn()?;

        // Capture serial output
        let output = self.capture_output(&mut qemu)?;

        // Parse test results
        self.parse_results(&output)
    }
}
```

#### QEMU Test Cases

```rust
#[test]
fn qemu_boot_and_initialize() {
    let harness = QemuTestHarness::new().unwrap();
    let result = harness.run_test("target/x86_64-unknown-uefi/release/basic.efi");
    assert!(result.success());
    assert!(result.output.contains("Allocator initialized"));
}

#[test]
fn qemu_protocol_availability() {
    let harness = QemuTestHarness::new().unwrap();
    let result = harness.run_test("target/x86_64-unknown-uefi/release/protocols.efi");
    assert!(result.success());
    assert!(result.output.contains("Text Output: Available"));
    assert!(result.output.contains("Graphics: Available"));
}

#[test]
fn qemu_file_operations() {
    let harness = QemuTestHarness::new().unwrap();
    let result = harness.run_test("target/x86_64-unknown-uefi/release/file_ops.efi");
    assert!(result.success());
    assert!(result.output.contains("File created successfully"));
}

#[test]
fn qemu_memory_stress() {
    let harness = QemuTestHarness::new().unwrap();
    let result = harness.run_test("target/x86_64-unknown-uefi/release/memory_stress.efi");
    assert!(result.success());
    assert!(result.output.contains("1000 allocations: OK"));
}
```

**Test Cases:** 10+ integration tests
- Boot sequence
- Protocol availability
- File system operations
- Memory allocation stress
- Network operations (simulated)
- Graphics operations
- Timer accuracy
- Event handling

---

## Phase 4: Property-Based Testing

### Test File: `tests/property_tests.rs`
**Enhancement:** ~300 lines

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_string_roundtrip(s in "\\PC*") {
        // UTF-8 → UCS-2 → UTF-8 should be identity
        let ucs2 = to_ucs2(&s);
        let back = from_ucs2(&ucs2);
        prop_assert_eq!(&s, &back);
    }

    #[test]
    fn prop_guid_parse_format(
        a in any::<u32>(),
        b in any::<u16>(),
        c in any::<u16>(),
        d in any::<[u8; 8]>()
    ) {
        // GUID format → parse → format should be identity
        let guid = Guid::new(a, b, c, d);
        let formatted = format!("{}", guid);
        let parsed = Guid::parse(&formatted).unwrap();
        prop_assert_eq!(guid, parsed);
    }

    #[test]
    fn prop_memory_alignment(
        size in 1..=4096usize,
        align in prop::sample::select(vec![8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096])
    ) {
        // All allocations should be properly aligned
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = unsafe { alloc::alloc::alloc(layout) };
        prop_assert!(!ptr.is_null());
        prop_assert_eq!(ptr as usize % align, 0);
        unsafe { alloc::alloc::dealloc(ptr, layout); }
    }

    #[test]
    fn prop_device_path_iteration(nodes in prop::collection::vec(any::<u8>(), 10..100)) {
        // Device path iteration should visit all nodes
        let path = create_device_path_from_bytes(&nodes);
        let collected: Vec<_> = path.iter().collect();
        // Should have at least one node
        prop_assert!(!collected.is_empty());
        // Last node should be END
        prop_assert!(collected.last().unwrap().is_end());
    }
}
```

**Property Tests:** 15+ properties
- String conversion round-trips
- GUID parsing/formatting
- Memory alignment invariants
- Device path iteration
- Event state transitions
- Protocol handle validity
- Variable name enumeration
- ACPI checksum validation

---

## Phase 5: Enhanced Unit Tests

### 5.1 Allocator Edge Cases

```rust
#[test]
fn test_allocator_oom() {
    // Test out-of-memory handling
    let mock_bs = create_mock_bs_with_limited_memory(1024);
    allocator::init_allocator(mock_bs);

    let huge = Layout::from_size_align(1_000_000, 8).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(huge) };
    assert!(ptr.is_null()); // Should return null, not panic
}

#[test]
fn test_allocator_high_alignment() {
    let layout = Layout::from_size_align(64, 4096).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(layout) };
    assert!(!ptr.is_null());
    assert_eq!(ptr as usize % 4096, 0);
    unsafe { alloc::alloc::dealloc(ptr, layout); }
}

#[test]
fn test_allocator_zero_size() {
    let layout = Layout::from_size_align(0, 8).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(layout) };
    // Zero-size allocation behavior is implementation-defined
    // Just ensure it doesn't crash
}

#[test]
fn test_allocator_fragmentation() {
    // Allocate and free in pattern to cause fragmentation
    let mut pointers = Vec::new();
    for _ in 0..100 {
        let layout = Layout::from_size_align(64, 8).unwrap();
        let ptr = unsafe { alloc::alloc::alloc(layout) };
        pointers.push((ptr, layout));
    }

    // Free every other allocation
    for (i, (ptr, layout)) in pointers.iter().enumerate() {
        if i % 2 == 0 {
            unsafe { alloc::alloc::dealloc(*ptr, *layout); }
        }
    }

    // Should still be able to allocate
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(layout) };
    assert!(!ptr.is_null());
}
```

---

### 5.2 Service Edge Cases

```rust
#[test]
fn test_variable_enumeration_empty() {
    let mock_vars = create_mock_variables_empty();
    let mut iter = mock_vars.enumerate();
    assert!(iter.next().is_none());
}

#[test]
fn test_variable_name_too_long() {
    let long_name = "a".repeat(1000);
    let result = vars.get(&long_name, &vendor_guid, &mut buffer);
    assert_eq!(result.unwrap_err(), EFI_INVALID_PARAMETER);
}

#[test]
fn test_time_invalid_values() {
    let mut time = EfiTime::default();
    time.month = 13; // Invalid
    let result = time_service.set_time(&time);
    assert_eq!(result.unwrap_err(), EFI_INVALID_PARAMETER);
}

#[test]
fn test_event_double_close() {
    let event = Event::new(boot_services, EVT_NOTIFY_SIGNAL)?;
    let handle = event.handle();
    drop(event); // First close

    // Attempt to close again should fail
    let result = unsafe {
        (boot_services.close_event)(handle)
    };
    assert_eq!(result, EFI_INVALID_PARAMETER);
}
```

---

### 5.3 Table Parsing Edge Cases

```rust
#[test]
fn test_acpi_invalid_checksum() {
    let mut rsdp = create_test_rsdp();
    rsdp.checksum = 0xFF; // Invalid
    let result = AcpiTableFinder::validate_rsdp(&rsdp);
    assert!(result.is_err());
}

#[test]
fn test_smbios_truncated_table() {
    let truncated_data = &VALID_SMBIOS_DATA[..10]; // Too short
    let iter = SmbiosIterator::new(truncated_data.as_ptr(), truncated_data.len());
    let entries: Vec<_> = iter.collect();
    // Should handle gracefully, not crash
    assert!(entries.is_empty() || entries[0].is_valid());
}

#[test]
fn test_acpi_table_iterator_empty() {
    let empty_xsdt = create_empty_xsdt();
    assert_eq!(empty_xsdt.entry_count(), 0);
    let result = empty_xsdt.get_entry(0);
    assert!(result.is_err());
}

#[test]
fn test_smbios_string_out_of_bounds() {
    let header = create_test_smbios_header();
    // Request string index that doesn't exist
    let result = header.get_string(99);
    assert_eq!(result, "");
}
```

---

## Test Coverage Goals

### By Module

| Module | Current | Target | Priority |
|--------|---------|--------|----------|
| Core Infrastructure | 80% | 90% | High |
| FFI Layer | 40% | 60% | Medium |
| Safe Wrappers | 70% | 85% | High |
| Protocols - Console | 75% | 85% | Medium |
| Protocols - Storage | 50% | 85% | **High** |
| Protocols - Network | 60% | 80% | High |
| Protocols - Security | 30% | 90% | **High** |
| Services - Boot | 70% | 85% | High |
| Services - Runtime | 65% | 85% | High |
| Tables - ACPI | 60% | 80% | Medium |
| Tables - SMBIOS | 60% | 80% | Medium |
| Utilities | 85% | 90% | Low |
| **Overall** | **60%** | **80%+** | - |

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: UEFI Rust Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly-2025-01-09
          target: x86_64-unknown-uefi
          components: rust-src

      - name: Run Unit Tests
        run: cargo test --tests

      - name: Install QEMU
        run: sudo apt-get install -y qemu-system-x86

      - name: Download OVMF
        run: |
          wget https://github.com/tianocore/edk2/releases/download/edk2-stable202308/OVMF-X64-r4.zip
          unzip OVMF-X64-r4.zip

      - name: Build Test Applications
        run: cargo build --release --target x86_64-unknown-uefi

      - name: Run QEMU Tests
        run: cargo test --test qemu_tests -- --test-threads=1

      - name: Generate Coverage Report
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml

      - name: Upload Coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml
```

---

## Test Maintenance

### Adding New Protocol Tests

When adding a new protocol, always include:

1. **Basic Functionality Test**
   - Verify protocol can be located
   - Test primary operations
   - Verify expected return values

2. **Error Path Tests**
   - Invalid parameters
   - Null pointers
   - Out-of-range values
   - Resource exhaustion

3. **Edge Case Tests**
   - Boundary values
   - Empty inputs
   - Maximum sizes
   - Concurrent access (if applicable)

4. **Mock Implementation**
   - Create in `tests/mock_environment.rs`
   - Implement all protocol functions
   - Validate parameters
   - Return realistic data

---

## Test Data Management

### Directory Structure

```
tests/
├── test_data/
│   ├── certificates/
│   │   ├── root_ca.der
│   │   ├── intermediate.der
│   │   └── end_entity.der
│   ├── signatures/
│   │   ├── valid_signature.p7b
│   │   └── invalid_signature.p7b
│   ├── acpi_tables/
│   │   ├── valid_rsdp.bin
│   │   └── valid_xsdt.bin
│   └── smbios/
│       └── valid_smbios.bin
└── README.md
```

### Generating Test Data

```bash
# Generate test certificates
openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -nodes

# Generate PKCS7 signature
openssl smime -sign -in data.txt -out signature.p7b -signer cert.pem -inkey key.pem -outform DER

# Create test ACPI tables
# (Use iasl or custom generator)
```

---

## Success Metrics

### Quantitative

- ✅ Overall coverage ≥ 80%
- ✅ Critical paths ≥ 95% coverage
- ✅ All protocols have ≥ 10 test cases
- ✅ All error paths tested
- ✅ QEMU tests pass on CI
- ✅ Zero known crashes
- ✅ No unsafe code without tests

### Qualitative

- ✅ Easy to add new tests
- ✅ Fast test execution (< 5 minutes)
- ✅ Clear test failure messages
- ✅ Comprehensive mock environment
- ✅ Well-documented test patterns

---

## Timeline

### Week 1-2: Storage & Security Tests
- Implement storage protocol tests (500 lines)
- Implement security protocol tests (600 lines)
- Create test data (certificates, signatures)
- Achieve 85%+ coverage for these modules

### Week 3: QEMU Integration
- Enhance QEMU test harness (300 lines)
- Create 10+ integration tests
- Set up CI/CD pipeline
- Automate OVMF download

### Week 4: Property Tests & Unit Tests
- Add 15+ property-based tests
- Expand unit test coverage
- Add edge case tests for all modules
- Achieve 80%+ overall coverage

### Week 5-6: Continuous Improvement
- Fix any failing tests
- Improve test documentation
- Optimize test performance
- Add missing test cases

---

## Conclusion

This comprehensive test plan will increase coverage from 60% to 80%+, with focus on critical paths and error handling. The multi-layered approach (unit, property, integration) ensures reliability without requiring UEFI hardware. Automated QEMU testing provides confidence that the code works in real UEFI environments.

**Key Deliverables:**
- 1,700+ new test lines
- 10+ QEMU integration tests
- 15+ property-based tests
- 80%+ overall coverage
- Automated CI/CD pipeline
- Comprehensive test data
