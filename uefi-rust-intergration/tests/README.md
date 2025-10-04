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
