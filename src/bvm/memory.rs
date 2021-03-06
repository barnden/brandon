extern crate byteorder;
#[path = "externals.rs"] mod externals;

use std::cell::RefCell;
use std::collections::HashMap;
use byteorder::{WriteBytesExt, BigEndian};
use self::externals::u64_to_u8arr;

pub struct Memory(RefCell<HashMap<u32, u64>>);

impl Memory {
    pub fn new() -> Memory {
        Memory (
            // Initialize with 2^16 memory locations
            RefCell::new(HashMap::with_capacity(65536))
        )
    }

    pub fn exists(&self, addr: &u32) -> bool {
        // Check to see if there exists data at this address
        self.0.borrow().contains_key(addr)
    }

    pub fn write(&self, addr: u32, content: u64) {
        // Write u64 content to this address
        &self.0.borrow_mut().insert(addr, content);
    }

    pub fn read(&self, addr: u32) -> Option<u64> {
        // Read data at this address
        if self.exists(&addr) {
            // If it exists then return the value stored at address
            Some(self.0.borrow()[&addr])
        } else {
            // Return none if not exists
            None
        }
    }

    pub fn delete(&self, addr: &u32) {
        // Deletes data at address
        if self.exists(&addr) {
            self.0.borrow_mut().remove_entry(addr);
        }
    }

    pub fn write_bytes(&self, start: u32, bytes: &[u8]) {
        // Writes bytes into memory
        let mut addr: u32 = start;
        
        for i in (0..bytes.len()).step_by(8) {
            let mut data: u64 = 0;

            for j in 0..8 {
                if i + j < bytes.len() {
                    data |= (bytes[i + j] as u64) << (56 - 8 * j);
                } else {
                    break;
                }
            }

            self.write(addr, data);
            addr += 1;
        }
    }

    pub fn read_bytes_eom(&self, start: u32) -> Vec<u8> {
        // Reads bytes into buffer until non existant or zero valued address.
        let mut addr: u32 = start;
        let mut buf: Vec<u8> = Vec::with_capacity(64);

        loop {
            // Read u64 from address
            let data: Option<u64> =  self.read(addr);
            match data {
                Some(_) => {
                    let data: u64 = data.unwrap();

                    if data == 0 {
                        break;
                    }

                    buf.extend_from_slice(&u64_to_u8arr(data));
                    addr += 1;
                },
                None => break
            }
        }

        buf
    }

    pub fn read_bytes(&self, start: u32, len: u32) -> Vec<u8> {
        // Reads len number of bytes into buffer.
        let mut addr: u32 = start;
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize);

        'memory: loop {
            // Read u64 from address
            let data: Option<u64> =  self.read(addr);
            match data {
                Some(_) => {
                    let data: u64 = data.unwrap();

                    let bytes = u64_to_u8arr(data);

                    for byte in &bytes {
                        buf.push(*byte);

                        if buf.len() == len as usize {
                            break 'memory;
                        }
                    }

                    addr += 1;
                },
                None => break 'memory
            }
        }

        buf
    }

    pub fn read_utf16(&self, start: u32) -> String {
        // Reads a UTF-16 BE string from memory, terminates at null byte.
        let bytes = self.read_bytes_eom(start);
        let mut chars: Vec<u16> = Vec::with_capacity(bytes.len() / 2);
        // It is OK to use half the length of bytes, because read_bytes
        // always emits bytes in multiples of 8.

        for i in (0..bytes.len()).step_by(2) {
            let word = ((bytes[i] as u16) << 8) | bytes[i + 1] as u16;

            if word == 0 { // Strings are terminated by null byte
                break;
            }

            chars.push(word);
        }

        String::from_utf16(&chars).unwrap()
    }

    pub fn write_utf16(&self, start: u32, string: String) {
        // Writes a UTF-16 BE string into memory.
        let chars: Vec<u16> = string.encode_utf16().collect();
        let mut bytes: Vec<u8> = Vec::with_capacity(chars.len() * 2);

        for chr in chars {
           let _ = bytes.write_u16::<BigEndian>(chr);
        }

        self.write_bytes(start, &bytes);
    }
}

#[test]
fn test_exists() {
    let mem = Memory::new();
    assert!(!mem.exists(&0x2929));

    mem.0.borrow_mut().insert(0x2929, 1);
    assert!(mem.exists(&0x2929));
}

#[test]
fn test_read() {
    let mem = Memory::new();
    mem.write(2929, 2929);

    assert_eq!(mem.read(29), None);
    assert_ne!(mem.read(2929), None);
    assert_eq!(mem.read(2929).unwrap(), 2929);

    mem.write(2929, 2930);

    assert_eq!(mem.read(2929).unwrap(), 2930);
}

#[test]
fn test_delete() {
    let mem = Memory::new();
    mem.write(2929, 2929);

    assert_eq!(mem.read(2929).unwrap(), 2929);

    mem.delete(&2929);

    assert_eq!(mem.read(2929), None);
}

#[test]
fn test_read_bytes_eom() {
    let mem = Memory::new();
    let correct: Vec<u8> = vec![0, 104, 0, 101, 0, 108, 0, 108, 0, 111, 0, 32, 0, 119, 0, 111, 0, 114, 0, 108, 0, 100, 0, 0];

    mem.write(0, 0x00680065006C006C);
    mem.write(1, 0x006F00200077006F);
    mem.write(2, 0x0072006C00640000);

    assert_eq!(correct, mem.read_bytes_eom(0));
}

#[test]
fn test_read_bytes() {
    let mem = Memory::new();
    let correct: Vec<u8> = vec![0, 111, 0, 32, 0, 119, 0, 111, 0];

    mem.write(0, 0x00680065006C006C);
    mem.write(1, 0x006F00200077006F);
    mem.write(2, 0x0072006C00640000);

    assert_eq!(vec![0, 0x68, 0], mem.read_bytes(0, 3));
    assert_eq!(correct, mem.read_bytes(1, 9));
}

#[test]
fn test_write_bytes() {
    let mem = Memory::new();
    let data: Vec<u8> = vec![0, 104, 0, 101, 0, 108, 0, 108, 0, 111, 0, 32, 0, 119, 0, 111, 0, 114, 0, 108, 0, 100, 0, 0];

    mem.write_bytes(3, &data);

    assert_eq!(mem.read(0), None);
    assert_eq!(mem.read(1), None);
    assert_eq!(mem.read(2), None);
    assert_eq!(mem.read(3).unwrap(), 0x00680065006C006C);
    assert_eq!(mem.read(4).unwrap(), 0x006F00200077006F);
    assert_eq!(mem.read(5).unwrap(), 0x0072006C00640000);
}

#[test]
fn test_read_utf16() {
    let mem = Memory::new();
    let correct: String = "hello world".to_owned();

    mem.write(0, 0x00680065006C006C);
    mem.write(1, 0x006F00200077006F);
    mem.write(2, 0x0072006C00640000);

    assert_eq!(correct, mem.read_utf16(0));
}

#[test]
fn test_write_utf16() {
    let mem = Memory::new();

    mem.write_utf16(0, "hello world".to_owned());

    let read = mem.read_utf16(0);

    assert_eq!(read, "hello world".to_owned());
}