// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Enhanced UEFI Application Example with Protocol Usage

#![no_std]
#![no_main]

extern crate alloc;
extern crate uefi_rust_intergration;

use alloc::vec::Vec;
use core::fmt::Write;
use uefi_rust_intergration::protocols::*;
use uefi_rust_intergration::*;

struct ConsoleWriter {
    console: *mut SimpleTextOutputProtocol,
}

impl Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let ucs2: Vec<u16> = s.encode_utf16().chain(core::iter::once(0)).collect();
        unsafe {
            let status = (*self.console).output_string(self.console, ucs2.as_ptr());
            if status == EFI_SUCCESS {
                Ok(())
            } else {
                Err(core::fmt::Error)
            }
        }
    }
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: *mut Handle,
    system_table: *mut SystemTable,
) -> Status {
    unsafe {
        let st = &*system_table;
        let bs = st.boot_services();

        // Initialize allocator
        allocator::init_allocator(bs);

        // Initialize panic handler
        panic_handler::init_panic_handler(st.con_out);

        // Initialize logger
        logger::Logger::init(st.con_out, logger::LogLevel::Debug);

        // Get console output
        let console = st.con_out;
        let mut writer = ConsoleWriter { console };

        // Clear screen
        (*console).clear_screen();

        // Set colors
        (*console).set_attribute(console, 0x0F); // White on black

        // Print banner
        let _ = writeln!(writer, "=================================================");
        let _ = writeln!(writer, "  UEFI Rust Integration - Protocol Demo");
        let _ = writeln!(
            writer,
            "=================================================\r\n"
        );

        log_info!("Application started");

        // Demonstrate Graphics Output Protocol
        let _ = writeln!(writer, "[1] Searching for Graphics Output Protocol...");
        let mut gop: *mut GraphicsOutputProtocol = core::ptr::null_mut();
        let status = (*bs).locate_protocol(
            &GRAPHICS_OUTPUT_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut gop as *mut *mut _ as *mut *mut core::ffi::c_void,
        );

        if status == EFI_SUCCESS && !gop.is_null() {
            let _ = writeln!(writer, "    [OK] Graphics Output Protocol found");
            let gop = &mut *gop;

            if let Some(info) = gop.current_mode_info() {
                let _ = writeln!(
                    writer,
                    "    Resolution: {}x{}",
                    info.horizontal_resolution, info.vertical_resolution
                );
                let _ = writeln!(writer, "    Pixel Format: {:?}", info.pixel_format);
            }

            if let Some(fb_base) = gop.frame_buffer_base() {
                let _ = writeln!(writer, "    Framebuffer Base: 0x{:016X}", fb_base);
            }
        } else {
            let _ = writeln!(writer, "    [FAIL] Graphics Output Protocol not found");
        }

        // Demonstrate Block I/O Protocol
        let _ = writeln!(writer, "\r\n[2] Searching for Block I/O Protocol...");
        let mut handles: *mut *mut Handle = core::ptr::null_mut();
        let mut handle_count: Uintn = 0;
        let status = (*bs).locate_handle_buffer(
            2, // ByProtocol
            &BLOCK_IO_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut handle_count,
            &mut handles,
        );

        if status == EFI_SUCCESS && !handles.is_null() {
            let _ = writeln!(
                writer,
                "    [OK] Found {} Block I/O device(s)",
                handle_count
            );

            for i in 0..handle_count.min(3) {
                let handle = *handles.add(i);
                let mut bio: *mut BlockIoProtocol = core::ptr::null_mut();
                let status = (*bs).handle_protocol(
                    handle,
                    &BLOCK_IO_PROTOCOL_GUID,
                    &mut bio as *mut *mut _ as *mut *mut core::ffi::c_void,
                );

                if status == EFI_SUCCESS && !bio.is_null() {
                    let bio = &*bio;
                    if let Some(media) = bio.media_info() {
                        let _ = writeln!(
                            writer,
                            "    Device {}: Block Size = {} bytes, Last Block = {}",
                            i, media.block_size, media.last_block
                        );
                    }
                }
            }

            // Free buffer
            (*bs).free_pool(handles as *mut core::ffi::c_void);
        } else {
            let _ = writeln!(writer, "    [FAIL] No Block I/O devices found");
        }

        // Demonstrate Simple File System Protocol
        let _ = writeln!(
            writer,
            "\r\n[3] Searching for Simple File System Protocol..."
        );
        let mut fs_handles: *mut *mut Handle = core::ptr::null_mut();
        let mut fs_count: Uintn = 0;
        let status = (*bs).locate_handle_buffer(
            2, // ByProtocol
            &SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut fs_count,
            &mut fs_handles,
        );

        if status == EFI_SUCCESS && !fs_handles.is_null() {
            let _ = writeln!(writer, "    [OK] Found {} file system(s)", fs_count);
            (*bs).free_pool(fs_handles as *mut core::ffi::c_void);
        } else {
            let _ = writeln!(writer, "    [FAIL] No file systems found");
        }

        // Demonstrate Loaded Image Protocol
        let _ = writeln!(writer, "\r\n[4] Getting Loaded Image Protocol...");
        let mut loaded_image: *mut LoadedImageProtocol = core::ptr::null_mut();
        let status = (*bs).handle_protocol(
            image_handle,
            &LOADED_IMAGE_PROTOCOL_GUID,
            &mut loaded_image as *mut *mut _ as *mut *mut core::ffi::c_void,
        );

        if status == EFI_SUCCESS && !loaded_image.is_null() {
            let _ = writeln!(writer, "    [OK] Loaded Image Protocol found");
            let img = &*loaded_image;
            let (base, size) = img.image_location();
            let _ = writeln!(writer, "    Image Base: {:?}", base);
            let _ = writeln!(writer, "    Image Size: {} bytes", size);
            let _ = writeln!(writer, "    Code Type: {:?}", img.image_code_type);
            let _ = writeln!(writer, "    Data Type: {:?}", img.image_data_type);
        } else {
            let _ = writeln!(writer, "    [FAIL] Could not get Loaded Image Protocol");
        }

        // Demonstrate PCI I/O Protocol
        let _ = writeln!(writer, "\r\n[5] Searching for PCI I/O Protocol...");
        let mut pci_handles: *mut *mut Handle = core::ptr::null_mut();
        let mut pci_count: Uintn = 0;
        let status = (*bs).locate_handle_buffer(
            2, // ByProtocol
            &PCI_IO_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut pci_count,
            &mut pci_handles,
        );

        if status == EFI_SUCCESS && !pci_handles.is_null() {
            let _ = writeln!(writer, "    [OK] Found {} PCI device(s)", pci_count);

            for i in 0..pci_count.min(3) {
                let handle = *pci_handles.add(i);
                let mut pci: *mut PciIoProtocol = core::ptr::null_mut();
                let status = (*bs).handle_protocol(
                    handle,
                    &PCI_IO_PROTOCOL_GUID,
                    &mut pci as *mut *mut _ as *mut *mut core::ffi::c_void,
                );

                if status == EFI_SUCCESS && !pci.is_null() {
                    let pci = &mut *pci;
                    if let Ok((seg, bus, dev, func)) = pci.get_location() {
                        let _ = writeln!(
                            writer,
                            "    Device {}: {:02X}:{:02X}.{:X}",
                            i, bus, dev, func
                        );
                    }
                }
            }

            (*bs).free_pool(pci_handles as *mut core::ffi::c_void);
        } else {
            let _ = writeln!(writer, "    [FAIL] No PCI devices found");
        }

        // Memory allocation demo
        let _ = writeln!(writer, "\r\n[6] Testing Memory Allocation...");
        log_debug!("Allocating test vector");
        let mut vec: Vec<u32> = Vec::new();
        for i in 0..10 {
            vec.push(i * i);
        }
        let _ = writeln!(writer, "    [OK] Allocated Vec with {} elements", vec.len());
        let _ = writeln!(writer, "    Data: {:?}", &vec[..5]);

        log_info!("All protocol demonstrations completed");

        // Wait for key
        let _ = writeln!(
            writer,
            "\r\n================================================="
        );
        let _ = writeln!(writer, "Press any key to exit...");
        let _ = writeln!(writer, "=================================================");

        let mut index: Uintn = 0;
        (*bs).wait_for_event(1, &mut (*st.con_in).wait_for_key, &mut index);

        EFI_SUCCESS
    }
}
