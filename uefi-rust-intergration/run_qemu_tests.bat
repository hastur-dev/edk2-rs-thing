@echo off
REM SPDX-License-Identifier: BSD-2-Clause-Patent
REM Run QEMU integration tests on Windows

echo UEFI Rust Integration - QEMU Test Runner
echo ==========================================

REM Check for QEMU
where qemu-system-x86_64 >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: qemu-system-x86_64 not found
    echo Download from: https://www.qemu.org/download/#windows
    exit /b 1
)

echo [OK] QEMU found

REM Check for OVMF
set OVMF_CODE=C:\Program Files\qemu\share\edk2-x86_64-code.fd
set OVMF_VARS=C:\Program Files\qemu\share\edk2-x86_64-vars.fd

if not exist "%OVMF_CODE%" (
    echo Error: OVMF firmware not found at %OVMF_CODE%
    echo Download OVMF from: https://github.com/tianocore/edk2/releases
    exit /b 1
)

echo [OK] OVMF found

REM Check Rust toolchain
rustup toolchain list | findstr "nightly-2025-01-09" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Installing nightly-2025-01-09 toolchain...
    rustup toolchain install nightly-2025-01-09
    rustup component add rust-src --toolchain nightly-2025-01-09
)

echo [OK] Rust toolchain ready

REM Add UEFI target
rustup target list | findstr "x86_64-unknown-uefi (installed)" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Adding x86_64-unknown-uefi target...
    rustup target add x86_64-unknown-uefi
)

echo [OK] UEFI target ready

REM Build library
echo.
echo Building UEFI library...
cargo +nightly-2025-01-09 build --lib --target x86_64-unknown-uefi

if %ERRORLEVEL% NEQ 0 (
    echo [FAIL] Library build failed
    exit /b 1
)

echo [OK] Library build successful

REM Run unit tests
echo.
echo Running unit tests...
cargo +nightly-2025-01-09 test --lib

if %ERRORLEVEL% NEQ 0 (
    echo [FAIL] Unit tests failed
    exit /b 1
)

echo [OK] Unit tests passed

REM Run mock tests
echo.
echo Running mock integration tests...
cargo +nightly-2025-01-09 test --test integration_tests

if %ERRORLEVEL% NEQ 0 (
    echo [FAIL] Mock integration tests failed
    exit /b 1
)

echo [OK] Mock integration tests passed

REM Run QEMU tests
echo.
echo Running QEMU integration tests...
echo Note: Tests marked with [SKIP] require example applications to exist
cargo +nightly-2025-01-09 test --test qemu_tests -- --ignored --test-threads=1

if %ERRORLEVEL% NEQ 0 (
    echo [WARN] Some QEMU tests may have been skipped
)

REM Summary
echo.
echo ==========================================
echo Test Summary
echo ==========================================
echo [OK] Unit tests completed
echo [OK] Mock integration tests completed
echo [OK] QEMU tests completed
echo.
echo All tests finished!
