// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Enhanced UEFI Panic Handler with Console Output

#[cfg(not(test))]
use crate::ffi::*;
use crate::protocols::SimpleTextOutputProtocol;
use core::fmt::Write;
use core::panic::PanicInfo;

static mut CONSOLE_OUT: Option<*mut SimpleTextOutputProtocol> = None;

/// Initialize panic handler with console output
///
/// # Safety
/// The console pointer must remain valid for the lifetime of the program
pub unsafe fn init_panic_handler(console: *mut SimpleTextOutputProtocol) {
    CONSOLE_OUT = Some(console);
}

#[cfg(not(test))]
#[allow(dead_code)]
struct PanicWriter {
    console: *mut SimpleTextOutputProtocol,
}

#[cfg(not(test))]
impl core::fmt::Write for PanicWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        use alloc::vec::Vec;

        if self.console.is_null() {
            return Err(core::fmt::Error);
        }

        // Convert to UCS-2
        let ucs2: Vec<u16> = s.encode_utf16().chain(core::iter::once(0)).collect();

        unsafe {
            let status = ((*self.console).output_string)(self.console, ucs2.as_ptr());
            if status == EFI_SUCCESS {
                Ok(())
            } else {
                Err(core::fmt::Error)
            }
        }
    }
}

/// Enhanced panic handler with console output
pub fn panic_handler(info: &PanicInfo) -> ! {
    unsafe {
        if let Some(console) = CONSOLE_OUT {
            let mut writer = PanicWriter { console };

            // Set red text on black background
            let _ = ((*console).set_attribute)(console, 0x04); // Red foreground

            let _ = writeln!(writer, "\r\n\r\n=== PANIC ===");

            if let Some(location) = info.location() {
                let _ = writeln!(
                    writer,
                    "Location: {}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                );
            }

            let _ = writeln!(writer, "Message: {}", info.message());

            let _ = writeln!(writer, "=== END PANIC ===\r\n");

            // Reset to default colors
            let _ = ((*console).set_attribute)(console, 0x07); // Light gray on black
        }
    }

    // Infinite loop
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
