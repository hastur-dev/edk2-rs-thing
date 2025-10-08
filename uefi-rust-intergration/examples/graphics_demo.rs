// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Graphics Demo Example
//!
//! This example demonstrates the Graphics Output Protocol (GOP) and
//! BMP image processing capabilities.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use uefi_rust::boot_services::*;
use uefi_rust::protocols::*;
use uefi_rust::*;

#[no_mangle]
pub extern "C" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    // Initialize allocator
    unsafe {
        uefi_rust::allocator::init(system_table);
    }

    // Get console output
    let st = unsafe { &mut *system_table };
    let con_out = unsafe { &mut *st.con_out };

    // Print banner
    print_string(con_out, "Graphics Demo\r\n");
    print_string(con_out, "=============\r\n\r\n");

    // Locate Graphics Output Protocol
    let boot_services = unsafe { &mut *st.boot_services };
    let mut gop_protocol: *mut GraphicsOutputProtocol = core::ptr::null_mut();

    let status = unsafe {
        (boot_services.locate_protocol)(
            &GRAPHICS_OUTPUT_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut gop_protocol as *mut _ as *mut *mut core::ffi::c_void,
        )
    };

    if status != EFI_SUCCESS {
        print_string(con_out, "Graphics Output Protocol not found\r\n");
        print_string(con_out, "This example requires GOP support\r\n");
        return status;
    }

    print_string(con_out, "Graphics Output Protocol located\r\n");

    let gop = unsafe { &mut *gop_protocol };
    let mode = unsafe { &*gop.mode };
    let info = unsafe { &*mode.info };

    // Display mode information
    print_string(con_out, "Current mode: ");
    print_number(con_out, mode.mode);
    print_string(con_out, "\r\n");

    print_string(con_out, "Resolution: ");
    print_number(con_out, info.horizontal_resolution);
    print_string(con_out, " x ");
    print_number(con_out, info.vertical_resolution);
    print_string(con_out, "\r\n");

    print_string(con_out, "Pixels per scan line: ");
    print_number(con_out, info.pixels_per_scan_line);
    print_string(con_out, "\r\n\r\n");

    // Draw some graphics
    print_string(con_out, "Drawing test pattern...\r\n");

    // Create a simple gradient pattern
    let width = 200u32;
    let height = 200u32;
    let mut blt_buffer: Vec<GraphicsOutputBltPixel> = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = GraphicsOutputBltPixel {
                blue: (x * 255 / width) as u8,
                green: (y * 255 / height) as u8,
                red: ((x + y) * 255 / (width + height)) as u8,
                reserved: 0,
            };
            blt_buffer.push(pixel);
        }
    }

    // Draw the pattern in the center of the screen
    let x = (info.horizontal_resolution - width) / 2;
    let y = (info.vertical_resolution - height) / 2;

    let status = unsafe {
        gop.blt(
            blt_buffer.as_ptr() as *mut _,
            EfiBltBufferToVideo,
            0,
            0,
            x,
            y,
            width,
            height,
            0,
        )
    };

    if status == EFI_SUCCESS {
        print_string(con_out, "Test pattern drawn successfully!\r\n");
    } else {
        print_string(con_out, "Failed to draw test pattern\r\n");
        return status;
    }

    // Draw a horizontal line
    print_string(con_out, "Drawing horizontal line...\r\n");

    let line_color = GraphicsOutputBltPixel {
        blue: 255,
        green: 255,
        red: 255,
        reserved: 0,
    };

    let status = unsafe {
        gop.blt(
            &line_color as *const _ as *mut _,
            EfiBltVideoFill,
            0,
            0,
            100,
            info.vertical_resolution / 2,
            info.horizontal_resolution - 200,
            5,
            0,
        )
    };

    if status == EFI_SUCCESS {
        print_string(con_out, "Line drawn successfully!\r\n");
    }

    // Draw a vertical line
    print_string(con_out, "Drawing vertical line...\r\n");

    let status = unsafe {
        gop.blt(
            &line_color as *const _ as *mut _,
            EfiBltVideoFill,
            0,
            0,
            info.horizontal_resolution / 2,
            100,
            5,
            info.vertical_resolution - 200,
            0,
        )
    };

    if status == EFI_SUCCESS {
        print_string(con_out, "Vertical line drawn!\r\n");
    }

    print_string(con_out, "\r\nGraphics demo completed!\r\n");
    print_string(con_out, "Press any key to continue...\r\n");

    // Wait for key press
    let stdin = unsafe { &mut *st.con_in };
    let mut key = InputKey {
        scan_code: 0,
        unicode_char: 0,
    };
    unsafe {
        (stdin.read_key_stroke)(stdin, &mut key);
    }

    EFI_SUCCESS
}

fn print_string(con_out: &mut SimpleTextOutputProtocol, s: &str) {
    let mut buf = [0u16; 256];
    let len = s.len().min(255);

    for (i, byte) in s.bytes().take(len).enumerate() {
        buf[i] = byte as u16;
    }
    buf[len] = 0;

    unsafe {
        (con_out.output_string)(con_out, buf.as_ptr() as *mut Char16);
    }
}

fn print_number(con_out: &mut SimpleTextOutputProtocol, n: u32) {
    let mut buf = [0u16; 32];
    let mut num = n;
    let mut i = 0;

    if num == 0 {
        buf[0] = '0' as u16;
        buf[1] = 0;
    } else {
        let mut digits = [0u16; 10];
        let mut count = 0;

        while num > 0 {
            digits[count] = (num % 10) as u16 + '0' as u16;
            num /= 10;
            count += 1;
        }

        for j in 0..count {
            buf[i] = digits[count - 1 - j];
            i += 1;
        }
        buf[i] = 0;
    }

    unsafe {
        (con_out.output_string)(con_out, buf.as_ptr() as *mut Char16);
    }
}
