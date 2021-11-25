use crate::cpu::Cpu;
use crate::iomap::IoMap;
use crate::ram::*;

pub struct Bus {
    ram: Ram,
    // ppu: Ppu,
    // apu: Apu,
}

impl IoMap for Bus {
    fn peek8(&self, address: u16) -> u8 {
        match address {
            0x00..=0x7ff => self.ram.peek8(address),
            _ => 0,
        }
    }
    fn poke8(&mut self, address: u16, byte: u8) {
        match address {
            0x00..=0x7ff => self.ram.poke8(address, byte),
            _ => (),
        }
    }
}
