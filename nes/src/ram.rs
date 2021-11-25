use crate::base::*;
use crate::iomap::IoMap;

pub struct Ram {
    bytes: [u8; SIZE_2K],
}

impl IoMap for Ram {
    fn peek8(&self, address: u16) -> u8 {
        self.bytes[address as usize & 0x7ff]
    }
    fn poke8(&mut self, address: u16, byte: u8) {
        self.bytes[address as usize & 0x7ff] = byte;
    }
}
