// SPDX-License-Identifier: BSD-2-Clause-Patent
//! BMP Graphics Library - Convert between BMP and GOP BLT buffer

use crate::protocols::graphics_output::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;

/// BMP File Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BmpFileHeader {
    pub signature: [u8; 2], // "BM"
    pub file_size: u32,
    pub reserved: u32,
    pub data_offset: u32,
}

/// BMP Info Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BmpInfoHeader {
    pub header_size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub image_size: u32,
    pub x_pixels_per_meter: i32,
    pub y_pixels_per_meter: i32,
    pub colors_used: u32,
    pub colors_important: u32,
}

/// BMP Color Palette Entry
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BmpColorMap {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

// BMP Compression types
pub const BI_RGB: u32 = 0;
pub const BI_RLE8: u32 = 1;
pub const BI_RLE4: u32 = 2;
pub const BI_BITFIELDS: u32 = 3;

/// BMP to BLT conversion errors
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BmpError {
    InvalidSignature,
    UnsupportedFormat,
    InvalidDimensions,
    BufferTooSmall,
    CompressionNotSupported,
}

/// BMP Image
pub struct BmpImage<'a> {
    file_header: &'a BmpFileHeader,
    info_header: &'a BmpInfoHeader,
    data: &'a [u8],
}

impl<'a> BmpImage<'a> {
    /// Parse a BMP file from a buffer
    pub fn from_buffer(buffer: &'a [u8]) -> Result<Self, BmpError> {
        if buffer.len()
            < core::mem::size_of::<BmpFileHeader>() + core::mem::size_of::<BmpInfoHeader>()
        {
            return Err(BmpError::BufferTooSmall);
        }

        let file_header = unsafe { &*(buffer.as_ptr() as *const BmpFileHeader) };

        if file_header.signature != *b"BM" {
            return Err(BmpError::InvalidSignature);
        }

        let info_header = unsafe {
            &*((buffer.as_ptr() as usize + core::mem::size_of::<BmpFileHeader>())
                as *const BmpInfoHeader)
        };

        // Validate dimensions
        if info_header.width <= 0 || info_header.height == 0 {
            return Err(BmpError::InvalidDimensions);
        }

        Ok(BmpImage {
            file_header,
            info_header,
            data: buffer,
        })
    }

    /// Get image width
    pub fn width(&self) -> u32 {
        self.info_header.width.unsigned_abs()
    }

    /// Get image height
    pub fn height(&self) -> u32 {
        self.info_header.height.unsigned_abs()
    }

    /// Check if image is bottom-up (height > 0) or top-down (height < 0)
    pub fn is_bottom_up(&self) -> bool {
        self.info_header.height > 0
    }

    /// Get bits per pixel
    pub fn bits_per_pixel(&self) -> u16 {
        self.info_header.bits_per_pixel
    }

    /// Convert BMP to GOP BLT buffer
    pub fn to_blt_buffer(&self) -> Result<(Vec<GraphicsOutputBltPixel>, u32, u32), BmpError> {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        // Only support 24-bit RGB for now
        if self.info_header.bits_per_pixel != 24 {
            return Err(BmpError::UnsupportedFormat);
        }

        if self.info_header.compression != BI_RGB {
            return Err(BmpError::CompressionNotSupported);
        }

        let width = self.width();
        let height = self.height();
        let mut blt_buffer = Vec::with_capacity((width * height) as usize);

        // Calculate row size (must be multiple of 4 bytes)
        let row_size = (self.info_header.bits_per_pixel as u32 * width).div_ceil(32) * 4;

        let pixel_data_start = self.file_header.data_offset as usize;

        for y in 0..height {
            // BMP is stored bottom-up by default
            let bmp_y = if self.is_bottom_up() {
                height - 1 - y
            } else {
                y
            };

            let row_offset = pixel_data_start + (bmp_y * row_size) as usize;

            for x in 0..width {
                let pixel_offset = row_offset + (x * 3) as usize;

                if pixel_offset + 2 >= self.data.len() {
                    return Err(BmpError::BufferTooSmall);
                }

                // BMP stores as BGR
                let blue = self.data[pixel_offset];
                let green = self.data[pixel_offset + 1];
                let red = self.data[pixel_offset + 2];

                blt_buffer.push(GraphicsOutputBltPixel {
                    blue,
                    green,
                    red,
                    reserved: 0,
                });
            }
        }

        Ok((blt_buffer, width, height))
    }

    /// Convert BMP to GOP BLT buffer with specified dimensions
    pub fn to_blt_buffer_scaled(
        &self,
        target_width: u32,
        target_height: u32,
    ) -> Result<Vec<GraphicsOutputBltPixel>, BmpError> {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        let (blt_buffer, width, height) = self.to_blt_buffer()?;

        if width == target_width && height == target_height {
            return Ok(blt_buffer);
        }

        // Simple nearest-neighbor scaling
        let mut scaled_buffer = Vec::with_capacity((target_width * target_height) as usize);

        for y in 0..target_height {
            let src_y = ((y as u64 * height as u64) / target_height as u64) as u32;

            for x in 0..target_width {
                let src_x = ((x as u64 * width as u64) / target_width as u64) as u32;
                let src_index = (src_y * width + src_x) as usize;

                scaled_buffer.push(blt_buffer[src_index]);
            }
        }

        Ok(scaled_buffer)
    }
}

/// Create a BMP file from a BLT buffer
pub fn blt_to_bmp(
    blt_buffer: &[GraphicsOutputBltPixel],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, BmpError> {
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;

    if width == 0 || height == 0 {
        return Err(BmpError::InvalidDimensions);
    }

    if blt_buffer.len() != (width * height) as usize {
        return Err(BmpError::BufferTooSmall);
    }

    // Calculate row size (must be multiple of 4 bytes)
    let row_size = (24 * width).div_ceil(32) * 4;
    let image_size = row_size * height;
    let file_size = core::mem::size_of::<BmpFileHeader>()
        + core::mem::size_of::<BmpInfoHeader>()
        + image_size as usize;

    let mut buffer = Vec::with_capacity(file_size);

    // File header
    let file_header = BmpFileHeader {
        signature: *b"BM",
        file_size: file_size as u32,
        reserved: 0,
        data_offset: (core::mem::size_of::<BmpFileHeader>() + core::mem::size_of::<BmpInfoHeader>())
            as u32,
    };

    // Info header
    let info_header = BmpInfoHeader {
        header_size: core::mem::size_of::<BmpInfoHeader>() as u32,
        width: width as i32,
        height: height as i32,
        planes: 1,
        bits_per_pixel: 24,
        compression: BI_RGB,
        image_size,
        x_pixels_per_meter: 0,
        y_pixels_per_meter: 0,
        colors_used: 0,
        colors_important: 0,
    };

    // Write headers
    unsafe {
        let fh_bytes = core::slice::from_raw_parts(
            &file_header as *const _ as *const u8,
            core::mem::size_of::<BmpFileHeader>(),
        );
        buffer.extend_from_slice(fh_bytes);

        let ih_bytes = core::slice::from_raw_parts(
            &info_header as *const _ as *const u8,
            core::mem::size_of::<BmpInfoHeader>(),
        );
        buffer.extend_from_slice(ih_bytes);
    }

    // Write pixel data (bottom-up)
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = &blt_buffer[(y * width + x) as usize];
            buffer.push(pixel.blue);
            buffer.push(pixel.green);
            buffer.push(pixel.red);
        }

        // Pad row to multiple of 4 bytes
        let padding = (4 - ((width * 3) % 4)) % 4;
        buffer.resize(buffer.len() + padding as usize, 0);
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmp_round_trip() {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        // Create a simple 2x2 image
        let blt_buffer = vec![
            GraphicsOutputBltPixel {
                blue: 255,
                green: 0,
                red: 0,
                reserved: 0,
            },
            GraphicsOutputBltPixel {
                blue: 0,
                green: 255,
                red: 0,
                reserved: 0,
            },
            GraphicsOutputBltPixel {
                blue: 0,
                green: 0,
                red: 255,
                reserved: 0,
            },
            GraphicsOutputBltPixel {
                blue: 255,
                green: 255,
                red: 255,
                reserved: 0,
            },
        ];

        let bmp_data = blt_to_bmp(&blt_buffer, 2, 2).unwrap();
        let bmp = BmpImage::from_buffer(&bmp_data).unwrap();

        assert_eq!(bmp.width(), 2);
        assert_eq!(bmp.height(), 2);
        assert_eq!(bmp.bits_per_pixel(), 24);

        let (decoded_buffer, width, height) = bmp.to_blt_buffer().unwrap();
        assert_eq!(width, 2);
        assert_eq!(height, 2);
        assert_eq!(decoded_buffer.len(), 4);
    }
}
