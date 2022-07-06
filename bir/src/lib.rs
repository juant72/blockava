type Hash=Vec<u8>;
type Address=String;

use std::time::{ SystemTime, UNIX_EPOCH};

pub fn now() -> u128{
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128

}

pub fn u32_bytes (u: &u32) -> [u8;4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bytes (u: &u64) -> [u8; 8]{
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]

}