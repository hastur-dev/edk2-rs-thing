// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Compiler Intrinsics for Multi-Architecture Support
//!
//! This module provides compiler intrinsics needed for no_std environments
//! across different architectures (x86, x86_64, aarch64, etc.)

/// Memory operations intrinsics
pub mod mem {
    /// memcpy implementation
    // Note: memcpy is now provided by compiler_builtins
    // #[no_mangle]
    // pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    //     let mut i = 0;
    //     while i < n {
    //         *dest.add(i) = *src.add(i);
    //         i += 1;
    //     }
    //     dest
    // }

    /// memmove implementation (handles overlapping memory)
    // Note: memmove is now provided by compiler_builtins
    // #[no_mangle]
    // pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    //     if src < dest as *const u8 {
    //         // Copy backwards to handle overlap
    //         let mut i = n;
    //         while i != 0 {
    //             i -= 1;
    //             *dest.add(i) = *src.add(i);
    //         }
    //     } else {
    //         // Copy forwards
    //         let mut i = 0;
    //         while i < n {
    //             *dest.add(i) = *src.add(i);
    //             i += 1;
    //         }
    //     }
    //     dest
    // }

    /// memset implementation
    // Note: memset is now provided by compiler_builtins
    // #[no_mangle]
    // pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    //     let mut i = 0;
    //     while i < n {
    //         *s.add(i) = c as u8;
    //         i += 1;
    //     }
    //     s
    // }

    /// memcmp implementation
    // Note: memcmp is now provided by compiler_builtins
    // #[no_mangle]
    // pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    //     let mut i = 0;
    //     while i < n {
    //         let a = *s1.add(i);
    //         let b = *s2.add(i);
    //         if a != b {
    //             return a as i32 - b as i32;
    //         }
    //         i += 1;
    //     }
    //     0
    // }

    /// bcmp implementation (same as memcmp for most purposes)
    #[no_mangle]
    pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
        let mut i = 0;
        while i < n {
            let a = *s1.add(i);
            let b = *s2.add(i);
            if a != b {
                return a as i32 - b as i32;
            }
            i += 1;
        }
        0
    }
}

/// Floating point intrinsics (basic support)
#[cfg(not(feature = "soft-float"))]
pub mod float {
    /// Float comparison: a < b
    #[no_mangle]
    pub extern "C" fn __lesf2(a: f32, b: f32) -> i32 {
        if a < b {
            -1
        } else if a == b {
            0
        } else {
            1
        }
    }

    /// Float comparison: a <= b
    #[no_mangle]
    pub extern "C" fn __ledf2(a: f64, b: f64) -> i32 {
        if a <= b {
            -1
        } else {
            1
        }
    }

    /// Float comparison: a > b
    #[no_mangle]
    pub extern "C" fn __gtsf2(a: f32, b: f32) -> i32 {
        if a > b {
            1
        } else if a == b {
            0
        } else {
            -1
        }
    }

    /// Float comparison: a >= b
    #[no_mangle]
    pub extern "C" fn __gtdf2(a: f64, b: f64) -> i32 {
        if a >= b {
            1
        } else {
            -1
        }
    }

    /// Float equality: a == b
    #[no_mangle]
    pub extern "C" fn __eqsf2(a: f32, b: f32) -> i32 {
        if a == b {
            0
        } else {
            1
        }
    }

    /// Double equality: a == b
    #[no_mangle]
    pub extern "C" fn __eqdf2(a: f64, b: f64) -> i32 {
        if a == b {
            0
        } else {
            1
        }
    }
}

/// 64-bit integer math intrinsics (for 32-bit platforms)
#[cfg(target_pointer_width = "32")]
pub mod math64 {
    /// 64-bit unsigned division
    #[no_mangle]
    pub extern "C" fn __udivdi3(a: u64, b: u64) -> u64 {
        a / b
    }

    /// 64-bit signed division
    #[no_mangle]
    pub extern "C" fn __divdi3(a: i64, b: i64) -> i64 {
        a / b
    }

    /// 64-bit unsigned modulo
    #[no_mangle]
    pub extern "C" fn __umoddi3(a: u64, b: u64) -> u64 {
        a % b
    }

    /// 64-bit signed modulo
    #[no_mangle]
    pub extern "C" fn __moddi3(a: i64, b: i64) -> i64 {
        a % b
    }

    /// 64-bit unsigned division and modulo
    #[no_mangle]
    pub extern "C" fn __udivmoddi4(a: u64, b: u64, rem: *mut u64) -> u64 {
        let quot = a / b;
        if !rem.is_null() {
            unsafe {
                *rem = a % b;
            }
        }
        quot
    }

    /// 64-bit multiplication
    #[no_mangle]
    pub extern "C" fn __muldi3(a: u64, b: u64) -> u64 {
        a.wrapping_mul(b)
    }
}

/// 128-bit integer math intrinsics (for 64-bit platforms)
#[cfg(target_pointer_width = "64")]
pub mod math128 {
    /// 128-bit unsigned division
    #[no_mangle]
    pub extern "C" fn __udivti3(a: u128, b: u128) -> u128 {
        a / b
    }

    /// 128-bit signed division
    #[no_mangle]
    pub extern "C" fn __divti3(a: i128, b: i128) -> i128 {
        a / b
    }

    /// 128-bit unsigned modulo
    #[no_mangle]
    pub extern "C" fn __umodti3(a: u128, b: u128) -> u128 {
        a % b
    }

    /// 128-bit signed modulo
    #[no_mangle]
    pub extern "C" fn __modti3(a: i128, b: i128) -> i128 {
        a % b
    }
}

/// Shift intrinsics
pub mod shift {
    /// 64-bit left shift
    #[no_mangle]
    pub extern "C" fn __ashldi3(a: u64, b: u32) -> u64 {
        a << b
    }

    /// 64-bit logical right shift
    #[no_mangle]
    pub extern "C" fn __lshrdi3(a: u64, b: u32) -> u64 {
        a >> b
    }

    /// 64-bit arithmetic right shift
    #[no_mangle]
    pub extern "C" fn __ashrdi3(a: i64, b: u32) -> i64 {
        a >> b
    }
}

/// Stack protection intrinsics (disabled in UEFI)
pub mod stack {
    /// Stack canary check (no-op in UEFI)
    #[no_mangle]
    pub extern "C" fn __stack_chk_fail() -> ! {
        // In UEFI, we can't really do much here
        // Just loop infinitely
        loop {
            #[cfg(target_arch = "x86_64")]
            unsafe {
                core::arch::asm!("hlt");
            }
            #[cfg(target_arch = "x86")]
            unsafe {
                core::arch::asm!("hlt");
            }
            #[cfg(target_arch = "aarch64")]
            unsafe {
                core::arch::asm!("wfi");
            }
        }
    }

    /// Stack canary guard (dummy in UEFI)
    #[no_mangle]
    pub static __stack_chk_guard: usize = 0xDEADBEEF;
}

/// Unwind/exception handling stubs (not supported in UEFI)
pub mod unwind {
    use core::ffi::c_void;

    #[repr(C)]
    pub struct _Unwind_Exception {
        _private: [u64; 2],
    }

    #[repr(C)]
    pub struct _Unwind_Context {
        _private: [u64; 2],
    }

    #[no_mangle]
    pub extern "C" fn _Unwind_Resume(_ex: *mut _Unwind_Exception) -> ! {
        loop {}
    }

    #[no_mangle]
    pub extern "C" fn __gxx_personality_v0(
        _version: i32,
        _actions: i32,
        _exception_class: u64,
        _exception_object: *mut _Unwind_Exception,
        _context: *mut _Unwind_Context,
    ) -> i32 {
        0
    }
}

/// Rust-specific compiler intrinsics
pub mod rust {
    /// Called when attempting to panic from a function marked as `nounwind`
    // Note: rust_begin_unwind is now provided by the panic handler
    // #[no_mangle]
    // pub extern "C" fn rust_begin_unwind(_info: &core::panic::PanicInfo) -> ! {
    //     loop {}
    // }

    /// Called for debug assertions that fail
    #[cfg(debug_assertions)]
    #[no_mangle]
    pub extern "C" fn __rust_probestack() {
        // No-op in UEFI
    }
}

/// Architecture-specific intrinsics
#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    use core::arch::asm;

    /// Read timestamp counter
    #[inline]
    pub unsafe fn rdtsc() -> u64 {
        let lo: u32;
        let hi: u32;
        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack)
        );
        ((hi as u64) << 32) | (lo as u64)
    }

    /// CPU pause instruction
    #[inline]
    pub unsafe fn cpu_pause() {
        asm!("pause", options(nomem, nostack));
    }

    /// Memory fence
    #[inline]
    pub unsafe fn mfence() {
        asm!("mfence", options(nostack));
    }

    /// Load fence
    #[inline]
    pub unsafe fn lfence() {
        asm!("lfence", options(nostack));
    }

    /// Store fence
    #[inline]
    pub unsafe fn sfence() {
        asm!("sfence", options(nostack));
    }
}

#[cfg(target_arch = "x86")]
pub mod x86 {
    use core::arch::asm;

    /// Read timestamp counter (32-bit version)
    #[inline]
    pub unsafe fn rdtsc() -> u64 {
        let lo: u32;
        let hi: u32;
        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack)
        );
        ((hi as u64) << 32) | (lo as u64)
    }

    /// CPU pause instruction
    #[inline]
    pub unsafe fn cpu_pause() {
        asm!("pause", options(nomem, nostack));
    }
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
    use core::arch::asm;

    /// Read system counter
    #[inline]
    pub unsafe fn read_system_counter() -> u64 {
        let cnt: u64;
        asm!("mrs {}, cntvct_el0", out(reg) cnt, options(nomem, nostack));
        cnt
    }

    /// Wait for interrupt
    #[inline]
    pub unsafe fn wfi() {
        asm!("wfi", options(nomem, nostack));
    }

    /// Wait for event
    #[inline]
    pub unsafe fn wfe() {
        asm!("wfe", options(nomem, nostack));
    }

    /// Send event
    #[inline]
    pub unsafe fn sev() {
        asm!("sev", options(nomem, nostack));
    }

    /// Data memory barrier
    #[inline]
    pub unsafe fn dmb() {
        asm!("dmb sy", options(nostack));
    }

    /// Data synchronization barrier
    #[inline]
    pub unsafe fn dsb() {
        asm!("dsb sy", options(nostack));
    }

    /// Instruction synchronization barrier
    #[inline]
    pub unsafe fn isb() {
        asm!("isb", options(nostack));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memcpy() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];
        unsafe {
            mem::memcpy(dst.as_mut_ptr(), src.as_ptr(), 5);
        }
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memset() {
        let mut buf = [0u8; 10];
        unsafe {
            mem::memset(buf.as_mut_ptr(), 0xAB, 10);
        }
        assert_eq!(buf, [0xAB; 10]);
    }

    #[test]
    fn test_memcmp() {
        let a = [1u8, 2, 3, 4, 5];
        let b = [1u8, 2, 3, 4, 5];
        let c = [1u8, 2, 3, 4, 6];

        unsafe {
            assert_eq!(mem::memcmp(a.as_ptr(), b.as_ptr(), 5), 0);
            assert!(mem::memcmp(a.as_ptr(), c.as_ptr(), 5) < 0);
        }
    }

    #[test]
    fn test_memmove_forward() {
        let mut buf = [1u8, 2, 3, 4, 5, 0, 0, 0];
        unsafe {
            mem::memmove(buf.as_mut_ptr().add(5), buf.as_ptr(), 3);
        }
        assert_eq!(buf, [1, 2, 3, 4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_memmove_backward() {
        let mut buf = [0u8, 0, 0, 1, 2, 3, 4, 5];
        unsafe {
            mem::memmove(buf.as_mut_ptr(), buf.as_ptr().add(3), 5);
        }
        assert_eq!(buf, [1, 2, 3, 4, 5, 3, 4, 5]);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_64bit_math() {
        let a: u64 = 100;
        let b: u64 = 7;
        assert_eq!(math64::__udivdi3(a, b), 14);
        assert_eq!(math64::__umoddi3(a, b), 2);
    }
}
