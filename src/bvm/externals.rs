extern crate libc;

use std::fs;
use libc::c_int;

extern "C" {
    fn getchar() -> c_int;
}

pub fn get_char() -> i32 {
    unsafe {
        getchar()
    }
}

pub fn read(path: &str) -> Vec<u8>{
    fs::read(path)
        .expect(
            &format!("Cannot open {}", path)
        )
}

pub fn u64_to_u8arr(int: u64) -> [u8; 8] {
    [
        (int >> 56 & 0xFF) as u8,
        (int >> 48 & 0xFF) as u8,
        (int >> 40 & 0xFF) as u8,
        (int >> 32 & 0xFF) as u8,
        (int >> 24 & 0xFF) as u8,
        (int >> 16 & 0xFF) as u8,
        (int >> 08 & 0xFF) as u8,
        (int       & 0xFF) as u8
    ]
}

#[test]
fn test_u64_to_u8arr() {
    let num: u64 = 0xF0E1D2C3B4A59687;
    let expected: [u8; 8] = [
        0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87
    ];

    assert_eq!(u64_to_u8arr(num), expected);
}