use crate::base::*;
use crate::iomap::IoMap;
use std::fmt;

pub struct Rom {
    bytes: [u8; SIZE_32K],
}

impl Rom {
    pub fn new() -> Self {
        Rom {
            bytes: [0; SIZE_32K],
        }
    }
    pub fn reset(&mut self) {
        self.bytes.fill(0);
    }
}

impl fmt::Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8_lossy(&self.bytes);
        write!(f, "{}", s)
    }
}

impl IoMap for Rom {
    fn peek8(&self, address: u16) -> u8 {
        self.bytes[address as usize & 0x7FF]
    }
    fn poke8(&mut self, address: u16, byte: u8) {
        self.bytes[address as usize & 0x7FF] = byte;
    }
}
