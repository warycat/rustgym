use crate::*;
use std::fmt;

pub struct PrgRom {
    mapper: u8,
    bytes: Vec<u8>,
}

impl PrgRom {
    pub fn new(mapper: u8, bytes: &[u8]) -> Self {
        let bytes = bytes.to_vec();
        PrgRom { mapper, bytes }
    }
    pub fn reset(&mut self) {
        self.bytes.fill(0);
    }
}

impl fmt::Debug for PrgRom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8_lossy(&self.bytes);
        write!(f, "{}", s)
    }
}

// impl MemoryHandler for PrgRom {
//     fn peek8(&mut self, address: u16) -> u8 {
//         let addr = address & 0x3FFF;
//         match self.mapper {
//             0 => self.bytes[addr as usize],
//             _ => 0,
//         }
//     }
//     fn poke8(&mut self, address: u16, byte: u8) {
//         let addr = address & 0x3FFF;
//         self.bytes[addr as usize] = byte;
//     }
// }
