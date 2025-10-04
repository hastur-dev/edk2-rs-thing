// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Miscellaneous UEFI Protocols - Timestamp, RNG, etc.

use crate::ffi::*;

/// EFI_TIMESTAMP_PROTOCOL_GUID
pub const TIMESTAMP_PROTOCOL_GUID: Guid = Guid::new(
    0xafbfde41,
    0x2e6e,
    0x4262,
    [0xba, 0x65, 0x62, 0xb9, 0x23, 0x6e, 0x54, 0x95],
);

/// EFI_RNG_PROTOCOL_GUID
pub const RNG_PROTOCOL_GUID: Guid = Guid::new(
    0x3152bca5,
    0xeade,
    0x433d,
    [0x86, 0x2e, 0xc0, 0x1c, 0xdc, 0x29, 0x1f, 0x44],
);

/// EFI_RNG_ALGORITHM_SP800_90_HASH_256_GUID
pub const RNG_ALGORITHM_SP800_90_HASH_256_GUID: Guid = Guid::new(
    0xa7af67cb,
    0x603b,
    0x4d42,
    [0xba, 0x21, 0x70, 0xbf, 0xb6, 0x29, 0x3f, 0x96],
);

/// EFI_RNG_ALGORITHM_SP800_90_HMAC_256_GUID
pub const RNG_ALGORITHM_SP800_90_HMAC_256_GUID: Guid = Guid::new(
    0xc5149b43,
    0xae85,
    0x4f53,
    [0x99, 0x82, 0xb9, 0x43, 0x35, 0xd3, 0xa9, 0xe7],
);

/// EFI_RNG_ALGORITHM_SP800_90_CTR_256_GUID
pub const RNG_ALGORITHM_SP800_90_CTR_256_GUID: Guid = Guid::new(
    0x44f0de6e,
    0x4d8c,
    0x4045,
    [0xa8, 0xc7, 0x4d, 0xd1, 0x68, 0x85, 0x6b, 0x9e],
);

/// EFI_RNG_ALGORITHM_X9_31_3DES_GUID
pub const RNG_ALGORITHM_X9_31_3DES_GUID: Guid = Guid::new(
    0x63c4785a,
    0xca34,
    0x4012,
    [0xa3, 0xc8, 0x0b, 0x6a, 0x32, 0x4f, 0x55, 0x46],
);

/// EFI_RNG_ALGORITHM_X9_31_AES_GUID
pub const RNG_ALGORITHM_X9_31_AES_GUID: Guid = Guid::new(
    0xacd03321,
    0x777e,
    0x4d3d,
    [0xb1, 0xc8, 0x20, 0xcf, 0xd8, 0x88, 0x20, 0xc9],
);

/// EFI_RNG_ALGORITHM_RAW
pub const RNG_ALGORITHM_RAW: Guid = Guid::new(
    0xe43176d7,
    0xb6e8,
    0x4827,
    [0xb7, 0x84, 0x7f, 0xfd, 0xc4, 0xb6, 0x85, 0x61],
);

/// Timestamp Properties
#[repr(C)]
pub struct TimestampProperties {
    pub frequency: Uint64,
    pub end_value: Uint64,
}

/// EFI_TIMESTAMP_PROTOCOL
#[repr(C)]
pub struct TimestampProtocol {
    pub get_timestamp: unsafe extern "efiapi" fn(this: *mut TimestampProtocol) -> Uint64,
    pub get_properties: unsafe extern "efiapi" fn(
        this: *mut TimestampProtocol,
        properties: *mut TimestampProperties,
    ) -> Status,
}

/// EFI_RNG_PROTOCOL
#[repr(C)]
pub struct RngProtocol {
    pub get_info: unsafe extern "efiapi" fn(
        this: *mut RngProtocol,
        rng_algorithm_list_size: *mut Uintn,
        rng_algorithm_list: *mut Guid,
    ) -> Status,
    pub get_rng: unsafe extern "efiapi" fn(
        this: *mut RngProtocol,
        rng_algorithm: *const Guid,
        rng_value_length: Uintn,
        rng_value: *mut Uint8,
    ) -> Status,
}

impl TimestampProtocol {
    /// Get current timestamp counter value
    pub unsafe fn get_timestamp(&mut self) -> u64 {
        (self.get_timestamp)(self)
    }

    /// Get timestamp properties (frequency and end value)
    pub unsafe fn get_properties(&mut self) -> Result<TimestampProperties, Status> {
        let mut props = core::mem::zeroed();
        let status = (self.get_properties)(self, &mut props);

        if status == EFI_SUCCESS {
            Ok(props)
        } else {
            Err(status)
        }
    }

    /// Calculate elapsed time in nanoseconds
    pub unsafe fn elapsed_ns(&mut self, start: u64, end: u64) -> Result<u64, Status> {
        let props = self.get_properties()?;
        if props.frequency == 0 {
            return Err(EFI_DEVICE_ERROR);
        }

        let ticks = if end >= start {
            end - start
        } else {
            // Handle counter wraparound
            (props.end_value - start) + end
        };

        // Convert ticks to nanoseconds: (ticks * 1_000_000_000) / frequency
        // Use 128-bit math to avoid overflow
        let ns = ((ticks as u128) * 1_000_000_000) / (props.frequency as u128);
        Ok(ns as u64)
    }
}

impl RngProtocol {
    /// Get list of supported RNG algorithms
    pub unsafe fn get_info(&mut self) -> Result<alloc::vec::Vec<Guid>, Status> {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        let mut size = 0;

        // First call to get size
        let _ = (self.get_info)(self, &mut size, core::ptr::null_mut());

        if size == 0 {
            return Ok(Vec::new());
        }

        let count = size / core::mem::size_of::<Guid>();
        let mut algorithms = Vec::with_capacity(count);
        algorithms.resize(count, Guid::new(0, 0, 0, [0; 8]));

        let status = (self.get_info)(self, &mut size, algorithms.as_mut_ptr());

        if status == EFI_SUCCESS {
            Ok(algorithms)
        } else {
            Err(status)
        }
    }

    /// Generate random bytes using specified algorithm
    pub unsafe fn get_rng(
        &mut self,
        algorithm: Option<&Guid>,
        buffer: &mut [u8],
    ) -> Status {
        let algo_ptr = algorithm
            .map(|a| a as *const _)
            .unwrap_or(core::ptr::null());
        (self.get_rng)(self, algo_ptr, buffer.len(), buffer.as_mut_ptr())
    }

    /// Generate random bytes using default algorithm
    pub unsafe fn get_random(&mut self, buffer: &mut [u8]) -> Status {
        self.get_rng(None, buffer)
    }

    /// Generate a random u32
    pub unsafe fn get_random_u32(&mut self) -> Result<u32, Status> {
        let mut buf = [0u8; 4];
        let status = self.get_random(&mut buf);

        if status == EFI_SUCCESS {
            Ok(u32::from_ne_bytes(buf))
        } else {
            Err(status)
        }
    }

    /// Generate a random u64
    pub unsafe fn get_random_u64(&mut self) -> Result<u64, Status> {
        let mut buf = [0u8; 8];
        let status = self.get_random(&mut buf);

        if status == EFI_SUCCESS {
            Ok(u64::from_ne_bytes(buf))
        } else {
            Err(status)
        }
    }
}

/// Helper module for timestamp utilities
pub mod timestamp_utils {
    use super::*;

    /// Timestamp measurement helper
    pub struct TimestampMeasure {
        start: u64,
        protocol: *mut TimestampProtocol,
    }

    impl TimestampMeasure {
        /// Start a new measurement
        pub unsafe fn start(protocol: *mut TimestampProtocol) -> Self {
            let start = ((*protocol).get_timestamp)(protocol);
            TimestampMeasure { start, protocol }
        }

        /// Get elapsed time in nanoseconds
        pub unsafe fn elapsed_ns(&mut self) -> Result<u64, Status> {
            let end = ((*self.protocol).get_timestamp)(self.protocol);
            (*self.protocol).elapsed_ns(self.start, end)
        }

        /// Get elapsed time in microseconds
        pub unsafe fn elapsed_us(&mut self) -> Result<u64, Status> {
            Ok(self.elapsed_ns()? / 1_000)
        }

        /// Get elapsed time in milliseconds
        pub unsafe fn elapsed_ms(&mut self) -> Result<u64, Status> {
            Ok(self.elapsed_ns()? / 1_000_000)
        }

        /// Restart the measurement
        pub unsafe fn restart(&mut self) {
            self.start = ((*self.protocol).get_timestamp)(self.protocol);
        }
    }
}

/// Helper module for RNG utilities
pub mod rng_utils {
    use super::*;

    /// Generate a random value in range [min, max]
    pub unsafe fn random_range(
        rng: &mut RngProtocol,
        min: u64,
        max: u64,
    ) -> Result<u64, Status> {
        if min >= max {
            return Err(EFI_INVALID_PARAMETER);
        }

        let range = max - min + 1;
        let random = rng.get_random_u64()?;
        Ok(min + (random % range))
    }

    /// Generate a random boolean
    pub unsafe fn random_bool(rng: &mut RngProtocol) -> Result<bool, Status> {
        let byte = rng.get_random_u32()?;
        Ok(byte & 1 == 1)
    }

    /// Fill a slice with random data
    pub unsafe fn fill_random(rng: &mut RngProtocol, buffer: &mut [u8]) -> Status {
        rng.get_random(buffer)
    }

    /// Shuffle a slice using Fisher-Yates algorithm
    pub unsafe fn shuffle<T>(rng: &mut RngProtocol, slice: &mut [T]) -> Result<(), Status> {
        let len = slice.len();
        if len <= 1 {
            return Ok(());
        }

        for i in (1..len).rev() {
            let j = random_range(rng, 0, i as u64)? as usize;
            slice.swap(i, j);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_properties_size() {
        assert!(core::mem::size_of::<TimestampProperties>() > 0);
    }

    #[test]
    fn test_rng_algorithm_guids() {
        // Ensure GUIDs are unique
        assert_ne!(
            RNG_ALGORITHM_SP800_90_HASH_256_GUID,
            RNG_ALGORITHM_SP800_90_HMAC_256_GUID
        );
    }

    #[test]
    fn test_elapsed_time_calculation() {
        // Mock test - just ensure the logic compiles
        let start = 100u64;
        let end = 200u64;
        let frequency = 1_000_000_000u64;

        let ticks = end - start;
        let ns = ((ticks as u128) * 1_000_000_000) / (frequency as u128);
        assert_eq!(ns, 100);
    }
}
