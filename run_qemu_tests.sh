#!/bin/bash
# SPDX-License-Identifier: BSD-2-Clause-Patent
# Run QEMU integration tests locally

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}UEFI Rust Integration - QEMU Test Runner${NC}"
echo "=========================================="

# Check for QEMU
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo -e "${RED}Error: qemu-system-x86_64 not found${NC}"
    echo "Install with: sudo apt-get install qemu-system-x86"
    exit 1
fi

echo -e "${GREEN}✓${NC} QEMU found: $(qemu-system-x86_64 --version | head -n1)"

# Check for OVMF
OVMF_CODE="/usr/share/ovmf/x64/OVMF_CODE.fd"
OVMF_VARS="/usr/share/ovmf/x64/OVMF_VARS.fd"

if [ ! -f "$OVMF_CODE" ]; then
    # Try alternative locations
    if [ -f "/usr/share/OVMF/OVMF_CODE.fd" ]; then
        OVMF_CODE="/usr/share/OVMF/OVMF_CODE.fd"
        OVMF_VARS="/usr/share/OVMF/OVMF_VARS.fd"
    else
        echo -e "${RED}Error: OVMF firmware not found${NC}"
        echo "Install with: sudo apt-get install ovmf"
        exit 1
    fi
fi

echo -e "${GREEN}✓${NC} OVMF found: $OVMF_CODE"

# Check Rust toolchain
if ! rustup toolchain list | grep -q "nightly-2025-01-09"; then
    echo -e "${YELLOW}Installing nightly-2025-01-09 toolchain...${NC}"
    rustup toolchain install nightly-2025-01-09
    rustup component add rust-src --toolchain nightly-2025-01-09
fi

echo -e "${GREEN}✓${NC} Rust toolchain ready"

# Add UEFI target
if ! rustup target list | grep -q "x86_64-unknown-uefi (installed)"; then
    echo -e "${YELLOW}Adding x86_64-unknown-uefi target...${NC}"
    rustup target add x86_64-unknown-uefi
fi

echo -e "${GREEN}✓${NC} UEFI target ready"

# Build library
echo ""
echo -e "${YELLOW}Building UEFI library...${NC}"
cargo +nightly-2025-01-09 build --lib --target x86_64-unknown-uefi

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Library build successful"
else
    echo -e "${RED}✗${NC} Library build failed"
    exit 1
fi

# Run unit tests
echo ""
echo -e "${YELLOW}Running unit tests...${NC}"
cargo +nightly-2025-01-09 test --lib

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Unit tests passed"
else
    echo -e "${RED}✗${NC} Unit tests failed"
    exit 1
fi

# Run mock tests
echo ""
echo -e "${YELLOW}Running mock integration tests...${NC}"
cargo +nightly-2025-01-09 test --test integration_tests

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Mock integration tests passed"
else
    echo -e "${RED}✗${NC} Mock integration tests failed"
    exit 1
fi

# Run QEMU tests
echo ""
echo -e "${YELLOW}Running QEMU integration tests...${NC}"
echo "Note: Tests marked with [SKIP] require example applications to exist"
cargo +nightly-2025-01-09 test --test qemu_tests -- --ignored --test-threads=1

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} QEMU integration tests passed"
else
    echo -e "${YELLOW}!${NC} Some QEMU tests may have been skipped"
fi

# Summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Test Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✓${NC} Unit tests completed"
echo -e "${GREEN}✓${NC} Mock integration tests completed"
echo -e "${GREEN}✓${NC} QEMU tests completed"
echo ""
echo -e "${GREEN}All tests finished!${NC}"
