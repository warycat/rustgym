use crate::cpu::Cpu;
use crate::iomap::IoMap;

#[derive(Debug)]
pub enum Addressing {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
    REL,
}

use Addressing::*;

impl Addressing {
    fn byte_size(&self) -> usize {
        match self {
            IMP => 0,
            IMM => 1,
            ZP0 => 1,
            ZPX => 1,
            ZPY => 1,
            ABS => 2,
            ABX => 1,
            ABY => 1,
            IND => 2,
            IZX => 1,
            IZY => 1,
            REL => 2,
        }
    }
}

pub trait AddressingMode {
    fn implied(&mut self);
    fn immediate(&mut self) -> u8;
    fn zero_page(&mut self) -> u16;
    fn zero_page_indexed(&mut self, indexed: u8) -> u16;
    fn absolute(&mut self) -> u16;
    fn indirect(&mut self) -> u16;
    fn absolute_indexed(&mut self, indexed: u8) -> u16;
    fn indirect_indexed(&mut self, indexed: u8) -> u16;
    fn indexed_indirect(&mut self, indexed: u8) -> u16;
    fn relative(&mut self) -> i8;
}

impl AddressingMode for Cpu {
    fn implied(&mut self) {}
    fn immediate(&mut self) -> u8 {
        let byte = self.fetch8();
        // cycles
        byte
    }
    fn zero_page(&mut self) -> u16 {
        let address = self.fetch8();
        address as u16
    }
    fn zero_page_indexed(&mut self, indexed: u8) -> u16 {
        let address = self.fetch8();
        let address = address.wrapping_add(indexed);
        address as u16
    }
    fn absolute(&mut self) -> u16 {
        let address = self.fetch16();
        address
    }
    fn indirect(&mut self) -> u16 {
        let addr = self.fetch16();
        let lo = self.peek8(addr) as u16;
        let hi = self.peek8(addr & 0xFF00 | (addr + 1) & 0x00FF) as u16;
        let address = lo | hi << 8;
        address
    }
    fn absolute_indexed(&mut self, indexed: u8) -> u16 {
        let address = self.fetch16();
        let address = address.wrapping_add(indexed as u16);
        address
    }
    fn indexed_indirect(&mut self, indexed: u8) -> u16 {
        let address = self.fetch8();
        let address = address.wrapping_add(indexed);
        let lo = self.peek8(address as u16) as u16;
        let address = address.wrapping_add(1);
        let hi = self.peek8(address as u16) as u16;
        lo | hi << 8
    }
    fn indirect_indexed(&mut self, indexed: u8) -> u16 {
        let address = self.fetch8();
        let lo = self.peek8(address as u16) as u16;
        let address = address.wrapping_add(1);
        let hi = self.peek8(address as u16) as u16;
        let address = lo | hi << 8;
        address.wrapping_add(indexed as u16)
    }
    fn relative(&mut self) -> i8 {
        let byte = self.fetch8();
        // cycles
        byte as i8
    }
}
