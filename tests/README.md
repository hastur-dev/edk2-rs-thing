# UEFI Rust Integration Tests

## Test Strategy

Since UEFI applications run in a special firmware environment, our testing strategy includes:

1. **Unit Tests**: Test individual components in isolation with mocked UEFI services
2. **Integration Tests**: Test component interactions with mock UEFI tables
3. **Property Tests**: Verify invariants and edge cases
4. **Compilation Tests**: Ensure no_std compatibility and correct ABI

## Test Categories

### 1. Type System Tests
- Verify struct layouts match UEFI spec
- Ensure correct sizes and alignments
- Test GUID comparisons and formatting

### 2. Allocator Tests
- Memory allocation and deallocation
- Out-of-memory handling
- Allocation alignment
- Memory leak detection

### 3. Boot Services Tests
- Function pointer validity
- Parameter validation
- Status code handling
- Protocol location

### 4. Runtime Services Tests
- Time services
- Variable services
- Reset functionality

### 5. Safe Wrapper Tests
- Error conversion
- Null pointer handling
- Result type correctness

### 6. Application Tests
- Entry point signature
- Initialization sequence
- Console output
- Cleanup and exit

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test --test ffi_tests

# Run with verbose output
cargo test -- --nocapture
```

## Mocking Strategy

Since we can't run UEFI code natively, we create mock implementations of:
- Boot Services function pointers
- Runtime Services function pointers
- System Table structures
- Protocol interfaces

## Test Structure

```
tests/
├── mock_uefi.rs              # Core UEFI mocking
├── mock_environment.rs       # Mock protocol implementations
├── ffi_tests.rs              # FFI layer tests
├── boot_services_tests.rs   # Boot Services tests
├── runtime_services_tests.rs# Runtime Services tests
├── allocator_tests.rs       # Allocator tests
├── protocol_tests.rs        # Protocol tests
├── services_tests.rs        # High-level service tests
├── storage_tests.rs         # Storage protocol tests
├── network_tests.rs         # Network protocol tests
├── graphics_tests.rs        # Graphics tests
├── table_tests.rs           # ACPI/SMBIOS table tests
├── property_tests.rs        # Property-based tests
├── integration_tests.rs     # Integration tests
├── qemu_runner.rs           # QEMU test harness
├── qemu_tests.rs            # QEMU integration tests
├── compilation_tests.rs     # Compilation tests
└── README.md                # This file
```

## Current Coverage

- **Overall:** ~60%
- **Target:** 80%+
- **Critical Paths:** High coverage
- **Error Paths:** Moderate coverage

See [TEST_PLAN.md](../TEST_PLAN.md) for detailed coverage goals and expansion plans.

## Future Testing

### Planned Additions

1. **Storage Protocol Tests** (~500 lines)
   - SCSI Pass Thru tests
   - NVMe Pass Thru tests
   - Disk I/O tests
   - Partition Info tests

2. **Security Protocol Tests** (~600 lines)
   - PKCS7 verification tests
   - Secure Boot tests
   - TPM 2.0 tests
   - Hash protocol tests

3. **QEMU Integration** (~300 lines)
   - Automated QEMU testing
   - Integration test suite
   - CI/CD integration

4. **Property-Based Tests** (~300 lines)
   - String conversion properties
   - GUID properties
   - Memory alignment properties
   - Protocol invariants

See [TEST_PLAN.md](../TEST_PLAN.md) for complete testing roadmap.

## Documentation

For more information on testing strategy and architecture:
- [TEST_PLAN.md](../TEST_PLAN.md) - Detailed test plan and coverage goals
- [ARCHITECTURE.md](../ARCHITECTURE.md) - System architecture and design
- [PROJECT_COMPLETION_PLAN.md](../PROJECT_COMPLETION_PLAN.md) - Project completion roadmap
