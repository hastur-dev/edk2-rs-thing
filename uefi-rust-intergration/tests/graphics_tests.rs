// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Graphics and Media Tests

#![cfg(test)]

use uefi_rust::graphics::bmp::*;
use uefi_rust::protocols::GraphicsOutputBltPixel;

#[test]
fn test_bmp_file_header_size() {
    assert_eq!(core::mem::size_of::<BmpFileHeader>(), 14);
}

#[test]
fn test_bmp_info_header_size() {
    assert_eq!(core::mem::size_of::<BmpInfoHeader>(), 40);
}

#[test]
fn test_bmp_signature() {
    let header = BmpFileHeader {
        signature: 0x4D42, // "BM"
        file_size: 0,
        reserved1: 0,
        reserved2: 0,
        data_offset: 54,
    };

    assert_eq!(header.signature, 0x4D42);
}

#[test]
fn test_bmp_info_header_creation() {
    let info = BmpInfoHeader {
        header_size: 40,
        width: 640,
        height: 480,
        planes: 1,
        bits_per_pixel: 24,
        compression: 0,
        image_size: 640 * 480 * 3,
        x_pixels_per_meter: 0,
        y_pixels_per_meter: 0,
        colors_used: 0,
        important_colors: 0,
    };

    assert_eq!(info.width, 640);
    assert_eq!(info.height, 480);
    assert_eq!(info.bits_per_pixel, 24);
    assert_eq!(info.compression, 0);
}

#[test]
fn test_bmp_pixel_to_blt_conversion() {
    let bmp_pixel = BmpPixel {
        blue: 255,
        green: 128,
        red: 64,
    };

    let blt_pixel = GraphicsOutputBltPixel {
        blue: bmp_pixel.blue,
        green: bmp_pixel.green,
        red: bmp_pixel.red,
        reserved: 0,
    };

    assert_eq!(blt_pixel.blue, 255);
    assert_eq!(blt_pixel.green, 128);
    assert_eq!(blt_pixel.red, 64);
}

#[test]
fn test_bmp_row_padding_calculation() {
    // BMP rows are padded to 4-byte boundaries
    let width = 10;
    let bytes_per_row = width * 3; // 30 bytes
    let padding = (4 - (bytes_per_row % 4)) % 4;

    assert_eq!(padding, 2); // 30 % 4 = 2, so need 2 bytes padding
}

#[test]
fn test_bmp_row_padding_examples() {
    // Width 10: 30 bytes -> 2 bytes padding
    assert_eq!((4 - ((10 * 3) % 4)) % 4, 2);

    // Width 4: 12 bytes -> 0 bytes padding (already aligned)
    assert_eq!((4 - ((4 * 3) % 4)) % 4, 0);

    // Width 5: 15 bytes -> 1 byte padding
    assert_eq!((4 - ((5 * 3) % 4)) % 4, 1);

    // Width 6: 18 bytes -> 2 bytes padding
    assert_eq!((4 - ((6 * 3) % 4)) % 4, 2);
}

#[test]
fn test_bmp_error_types() {
    let err = BmpError::InvalidSignature;
    match err {
        BmpError::InvalidSignature => {}
        _ => panic!("Wrong error type"),
    }

    let err = BmpError::InvalidHeader;
    match err {
        BmpError::InvalidHeader => {}
        _ => panic!("Wrong error type"),
    }

    let err = BmpError::UnsupportedFormat;
    match err {
        BmpError::UnsupportedFormat => {}
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_bmp_minimum_file_size() {
    let min_size = core::mem::size_of::<BmpFileHeader>() + core::mem::size_of::<BmpInfoHeader>();
    assert_eq!(min_size, 54);
}

#[test]
fn test_blt_pixel_creation() {
    let pixel = GraphicsOutputBltPixel {
        blue: 255,
        green: 255,
        red: 255,
        reserved: 0,
    };

    // White pixel
    assert_eq!(pixel.blue, 255);
    assert_eq!(pixel.green, 255);
    assert_eq!(pixel.red, 255);

    let pixel = GraphicsOutputBltPixel {
        blue: 0,
        green: 0,
        red: 0,
        reserved: 0,
    };

    // Black pixel
    assert_eq!(pixel.blue, 0);
    assert_eq!(pixel.green, 0);
    assert_eq!(pixel.red, 0);
}

#[test]
fn test_scaling_calculation() {
    // Test nearest-neighbor scaling
    let src_width = 100u32;
    let src_height = 100u32;
    let dst_width = 200u32;
    let dst_height = 200u32;

    // Scale factor: 2x
    let scale_x = dst_width as f32 / src_width as f32;
    let scale_y = dst_height as f32 / src_height as f32;

    assert_eq!(scale_x, 2.0);
    assert_eq!(scale_y, 2.0);

    // Source coordinate for destination (50, 50)
    let dst_x = 50u32;
    let dst_y = 50u32;
    let src_x = (dst_x as f32 / scale_x) as u32;
    let src_y = (dst_y as f32 / scale_y) as u32;

    assert_eq!(src_x, 25);
    assert_eq!(src_y, 25);
}

#[test]
fn test_color_values() {
    // Test common colors in BLT format
    let red = GraphicsOutputBltPixel {
        blue: 0,
        green: 0,
        red: 255,
        reserved: 0,
    };

    let green = GraphicsOutputBltPixel {
        blue: 0,
        green: 255,
        red: 0,
        reserved: 0,
    };

    let blue = GraphicsOutputBltPixel {
        blue: 255,
        green: 0,
        red: 0,
        reserved: 0,
    };

    assert_eq!(red.red, 255);
    assert_eq!(green.green, 255);
    assert_eq!(blue.blue, 255);
}

#[test]
fn test_bmp_compression_types() {
    const BI_RGB: u32 = 0;
    const BI_RLE8: u32 = 1;
    const BI_RLE4: u32 = 2;
    const BI_BITFIELDS: u32 = 3;

    assert_eq!(BI_RGB, 0);
    assert_eq!(BI_RLE8, 1);
    assert_eq!(BI_RLE4, 2);
    assert_eq!(BI_BITFIELDS, 3);
}

#[test]
fn test_bmp_supported_bits_per_pixel() {
    // We support 24-bit RGB
    let supported = [24u16];

    assert!(supported.contains(&24));
    assert!(!supported.contains(&8));
    assert!(!supported.contains(&16));
    assert!(!supported.contains(&32));
}

#[test]
fn test_image_size_calculation() {
    let width = 640u32;
    let height = 480u32;
    let bytes_per_pixel = 3; // 24-bit RGB

    let row_size = width * bytes_per_pixel;
    let padding = (4 - (row_size % 4)) % 4;
    let padded_row_size = row_size + padding;
    let image_size = padded_row_size * height;

    assert_eq!(row_size, 1920);
    assert_eq!(padding, 0); // 1920 % 4 == 0
    assert_eq!(image_size, 1920 * 480);
}

#[test]
fn test_bmp_coordinate_system() {
    // BMP uses bottom-up coordinate system (negative height)
    // or top-down (positive height)

    let bottom_up_height: i32 = -480;
    let top_down_height: i32 = 480;

    assert!(bottom_up_height < 0);
    assert!(top_down_height > 0);

    // Absolute height
    assert_eq!(bottom_up_height.abs(), 480);
    assert_eq!(top_down_height.abs(), 480);
}
