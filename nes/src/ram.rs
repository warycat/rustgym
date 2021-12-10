use crate::base::*;
use crate::iomap::IoMap;
use std::fmt;

pub struct Ram {
    bytes: [u8; SIZE_2K],
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            bytes: [0; SIZE_2K],
        }
    }
    pub fn reset(&mut self) {
        self.bytes.fill(0);
    }
}

impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8_lossy(&self.bytes);
        write!(f, "{}", s)
    }
}

impl IoMap for Ram {
    fn peek8(&self, address: u16) -> u8 {
        self.bytes[address as usize & 0x7FF]
    }
    fn poke8(&mut self, address: u16, byte: u8) {
        self.bytes[address as usize & 0x7FF] = byte;
    }
}
