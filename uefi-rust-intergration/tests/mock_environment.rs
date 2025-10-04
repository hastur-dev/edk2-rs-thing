// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Mock UEFI Environment for Testing
//!
//! This module provides a complete mock UEFI environment for testing
//! protocols and services without requiring actual UEFI firmware.

#![cfg(test)]

use uefi_rust::ffi::*;
use uefi_rust::protocols::*;
use uefi_rust::boot_services::*;
use uefi_rust::runtime_services::*;
use std::sync::Mutex;
use std::collections::HashMap;

/// Mock UEFI environment
pub struct MockUefiEnvironment {
    pub boot_services: MockBootServices,
    pub runtime_services: MockRuntimeServices,
    pub protocols: HashMap<Guid, *mut core::ffi::c_void>,
}

impl MockUefiEnvironment {
    pub fn new() -> Self {
        MockUefiEnvironment {
            boot_services: MockBootServices::new(),
            runtime_services: MockRuntimeServices::new(),
            protocols: HashMap::new(),
        }
    }

    pub fn install_protocol<T>(&mut self, guid: Guid, protocol: *mut T) {
        self.protocols.insert(guid, protocol as *mut core::ffi::c_void);
    }

    pub fn locate_protocol<T>(&self, guid: &Guid) -> Option<*mut T> {
        self.protocols.get(guid).map(|p| *p as *mut T)
    }
}

/// Mock Boot Services
pub struct MockBootServices {
    pub memory_allocations: Mutex<Vec<(*mut u8, usize)>>,
    pub events: Mutex<Vec<MockEvent>>,
    pub current_tpl: Mutex<Tpl>,
}

impl MockBootServices {
    pub fn new() -> Self {
        MockBootServices {
            memory_allocations: Mutex::new(Vec::new()),
            events: Mutex::new(Vec::new()),
            current_tpl: Mutex::new(4), // TPL_APPLICATION
        }
    }

    pub fn allocate_pool(&self, size: usize) -> *mut u8 {
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        self.memory_allocations.lock().unwrap().push((ptr, size));
        ptr
    }

    pub fn free_pool(&self, ptr: *mut u8) {
        let mut allocations = self.memory_allocations.lock().unwrap();
        if let Some(pos) = allocations.iter().position(|(p, _)| *p == ptr) {
            let (ptr, size) = allocations.remove(pos);
            let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
            unsafe {
                std::alloc::dealloc(ptr, layout);
            }
        }
    }

    pub fn create_event(&self, event_type: u32, notify_tpl: Tpl) -> MockEventHandle {
        let event = MockEvent {
            event_type,
            notify_tpl,
            signaled: false,
        };
        let mut events = self.events.lock().unwrap();
        events.push(event);
        MockEventHandle(events.len() - 1)
    }

    pub fn close_event(&self, handle: MockEventHandle) {
        let mut events = self.events.lock().unwrap();
        if handle.0 < events.len() {
            events.remove(handle.0);
        }
    }

    pub fn raise_tpl(&self, new_tpl: Tpl) -> Tpl {
        let mut current = self.current_tpl.lock().unwrap();
        let old_tpl = *current;
        *current = new_tpl;
        old_tpl
    }

    pub fn restore_tpl(&self, old_tpl: Tpl) {
        *self.current_tpl.lock().unwrap() = old_tpl;
    }

    pub fn get_current_tpl(&self) -> Tpl {
        *self.current_tpl.lock().unwrap()
    }
}

/// Mock Event
#[derive(Clone)]
pub struct MockEvent {
    pub event_type: u32,
    pub notify_tpl: Tpl,
    pub signaled: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct MockEventHandle(usize);

/// Mock Runtime Services
pub struct MockRuntimeServices {
    pub variables: Mutex<HashMap<(String, Guid), Vec<u8>>>,
    pub time: Mutex<Time>,
}

impl MockRuntimeServices {
    pub fn new() -> Self {
        MockRuntimeServices {
            variables: Mutex::new(HashMap::new()),
            time: Mutex::new(Time {
                year: 2025,
                month: 10,
                day: 4,
                hour: 12,
                minute: 0,
                second: 0,
                pad1: 0,
                nanosecond: 0,
                time_zone: 0,
                daylight: 0,
                pad2: 0,
            }),
        }
    }

    pub fn set_variable(&self, name: &str, guid: &Guid, data: Vec<u8>) {
        let key = (name.to_string(), *guid);
        self.variables.lock().unwrap().insert(key, data);
    }

    pub fn get_variable(&self, name: &str, guid: &Guid) -> Option<Vec<u8>> {
        let key = (name.to_string(), *guid);
        self.variables.lock().unwrap().get(&key).cloned()
    }

    pub fn delete_variable(&self, name: &str, guid: &Guid) {
        let key = (name.to_string(), *guid);
        self.variables.lock().unwrap().remove(&key);
    }

    pub fn get_time(&self) -> Time {
        *self.time.lock().unwrap()
    }

    pub fn set_time(&self, time: Time) {
        *self.time.lock().unwrap() = time;
    }
}

/// Mock Simple Text Output Protocol
pub struct MockSimpleTextOutput {
    pub output: Mutex<String>,
}

impl MockSimpleTextOutput {
    pub fn new() -> Self {
        MockSimpleTextOutput {
            output: Mutex::new(String::new()),
        }
    }

    pub fn get_output(&self) -> String {
        self.output.lock().unwrap().clone()
    }

    pub fn clear_output(&self) {
        self.output.lock().unwrap().clear();
    }

    pub unsafe extern "efiapi" fn mock_output_string(
        this: *mut SimpleTextOutputProtocol,
        string: *mut Char16,
    ) -> Status {
        // Convert UCS-2 to String
        let mut chars = Vec::new();
        let mut ptr = string;
        while !ptr.is_null() && *ptr != 0 {
            chars.push(*ptr);
            ptr = ptr.add(1);
        }
        let text = String::from_utf16_lossy(&chars);

        // This is a simplified version - in real implementation would store output
        println!("Mock output: {}", text);
        EFI_SUCCESS
    }
}

/// Mock Block I/O Protocol
pub struct MockBlockIo {
    pub media: BlockIoMedia,
    pub data: Mutex<Vec<u8>>,
}

impl MockBlockIo {
    pub fn new(block_count: u64, block_size: u32) -> Self {
        let total_size = (block_count * block_size as u64) as usize;
        MockBlockIo {
            media: BlockIoMedia {
                media_id: 1,
                removable_media: 0,
                media_present: 1,
                logical_partition: 0,
                read_only: 0,
                write_caching: 0,
                block_size,
                io_align: 1,
                last_block: block_count - 1,
                lowest_aligned_lba: 0,
                logical_blocks_per_physical_block: 1,
                optimal_transfer_length_granularity: 1,
            },
            data: Mutex::new(vec![0u8; total_size]),
        }
    }

    pub fn read_blocks(&self, lba: u64, buffer: &mut [u8]) -> Result<(), &'static str> {
        let offset = (lba * self.media.block_size as u64) as usize;
        let data = self.data.lock().unwrap();

        if offset + buffer.len() > data.len() {
            return Err("Read beyond end of device");
        }

        buffer.copy_from_slice(&data[offset..offset + buffer.len()]);
        Ok(())
    }

    pub fn write_blocks(&self, lba: u64, buffer: &[u8]) -> Result<(), &'static str> {
        let offset = (lba * self.media.block_size as u64) as usize;
        let mut data = self.data.lock().unwrap();

        if offset + buffer.len() > data.len() {
            return Err("Write beyond end of device");
        }

        data[offset..offset + buffer.len()].copy_from_slice(buffer);
        Ok(())
    }
}

/// Mock Network Interface
pub struct MockNetworkInterface {
    pub mac_address: [u8; 6],
    pub tx_packets: Mutex<Vec<Vec<u8>>>,
    pub rx_packets: Mutex<Vec<Vec<u8>>>,
}

impl MockNetworkInterface {
    pub fn new(mac: [u8; 6]) -> Self {
        MockNetworkInterface {
            mac_address: mac,
            tx_packets: Mutex::new(Vec::new()),
            rx_packets: Mutex::new(Vec::new()),
        }
    }

    pub fn transmit(&self, packet: Vec<u8>) {
        self.tx_packets.lock().unwrap().push(packet);
    }

    pub fn receive(&self) -> Option<Vec<u8>> {
        self.rx_packets.lock().unwrap().pop()
    }

    pub fn inject_packet(&self, packet: Vec<u8>) {
        self.rx_packets.lock().unwrap().push(packet);
    }

    pub fn get_transmitted_packets(&self) -> Vec<Vec<u8>> {
        self.tx_packets.lock().unwrap().clone()
    }
}

// Tests using mock environment
#[test]
fn test_mock_boot_services_memory() {
    let bs = MockBootServices::new();

    let ptr = bs.allocate_pool(1024);
    assert!(!ptr.is_null());

    bs.free_pool(ptr);

    // Verify allocations are tracked
    assert_eq!(bs.memory_allocations.lock().unwrap().len(), 0);
}

#[test]
fn test_mock_boot_services_tpl() {
    let bs = MockBootServices::new();

    assert_eq!(bs.get_current_tpl(), 4); // TPL_APPLICATION

    let old_tpl = bs.raise_tpl(16); // TPL_NOTIFY
    assert_eq!(old_tpl, 4);
    assert_eq!(bs.get_current_tpl(), 16);

    bs.restore_tpl(old_tpl);
    assert_eq!(bs.get_current_tpl(), 4);
}

#[test]
fn test_mock_runtime_services_variables() {
    let rs = MockRuntimeServices::new();

    let guid = Guid::new(1, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);
    let data = vec![1, 2, 3, 4, 5];

    rs.set_variable("TestVar", &guid, data.clone());

    let retrieved = rs.get_variable("TestVar", &guid);
    assert_eq!(retrieved, Some(data));

    rs.delete_variable("TestVar", &guid);
    assert_eq!(rs.get_variable("TestVar", &guid), None);
}

#[test]
fn test_mock_runtime_services_time() {
    let rs = MockRuntimeServices::new();

    let time = rs.get_time();
    assert_eq!(time.year, 2025);

    let new_time = Time {
        year: 2026,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        pad1: 0,
        nanosecond: 0,
        time_zone: 0,
        daylight: 0,
        pad2: 0,
    };

    rs.set_time(new_time);
    let retrieved = rs.get_time();
    assert_eq!(retrieved.year, 2026);
}

#[test]
fn test_mock_block_io() {
    let block_io = MockBlockIo::new(100, 512);

    // Write some data
    let write_data = vec![0xAA; 512];
    assert!(block_io.write_blocks(0, &write_data).is_ok());

    // Read it back
    let mut read_data = vec![0; 512];
    assert!(block_io.read_blocks(0, &mut read_data).is_ok());

    assert_eq!(read_data, write_data);
}

#[test]
fn test_mock_network_interface() {
    let net = MockNetworkInterface::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

    assert_eq!(net.mac_address, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

    // Transmit a packet
    let packet = vec![1, 2, 3, 4, 5];
    net.transmit(packet.clone());

    let transmitted = net.get_transmitted_packets();
    assert_eq!(transmitted.len(), 1);
    assert_eq!(transmitted[0], packet);

    // Inject and receive a packet
    let rx_packet = vec![6, 7, 8, 9, 10];
    net.inject_packet(rx_packet.clone());

    let received = net.receive();
    assert_eq!(received, Some(rx_packet));
}

#[test]
fn test_mock_environment_protocol_installation() {
    let mut env = MockUefiEnvironment::new();

    let guid = SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID;
    let protocol_ptr = Box::into_raw(Box::new(MockSimpleTextOutput::new()));

    env.install_protocol(guid, protocol_ptr);

    let retrieved: Option<*mut MockSimpleTextOutput> = env.locate_protocol(&guid);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), protocol_ptr);

    // Cleanup
    unsafe {
        let _ = Box::from_raw(protocol_ptr);
    }
}
