// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Basic UEFI types

/// BOOLEAN type
pub type Boolean = u8;

/// INTN type
pub type Intn = isize;

/// UINTN type
pub type Uintn = usize;

/// INT8
pub type Int8 = i8;

/// UINT8
pub type Uint8 = u8;

/// INT16
pub type Int16 = i16;

/// UINT16
pub type Uint16 = u16;

/// INT32
pub type Int32 = i32;

/// UINT32
pub type Uint32 = u32;

/// INT64
pub type Int64 = i64;

/// UINT64
pub type Uint64 = u64;

/// CHAR8
pub type Char8 = u8;

/// CHAR16
pub type Char16 = u16;

/// EFI_STATUS
pub type Status = usize;

pub const FALSE: Boolean = 0;
pub const TRUE: Boolean = 1;
