use crate::cpu::Cpu;
use crate::iomap::IoMap;

pub trait Addressing {
    fn immediate_read(&mut self) -> u8;
    fn zero_page_read(&mut self) -> u8;
    fn zero_page_indexed_read(&mut self, indexed: u8) -> u8;
    fn absolute_read(&mut self) -> u8;
    fn absolute_indexed_read(&mut self, indexed: u8) -> u8;
}

impl Addressing for Cpu {
    fn immediate_read(&mut self) -> u8 {
        let byte = self.fetch8();
        // cycles
        byte
    }
    fn zero_page_read(&mut self) -> u8 {
        let address = self.fetch8() as u16;
        // cycles
        self.peek8(address)
    }
    fn zero_page_indexed_read(&mut self, indexed: u8) -> u8 {
        let address = self.fetch8() + indexed;
        // cycles
        self.peek8(address as u16)
    }
    fn absolute_read(&mut self) -> u8 {
        let address = self.fetch16();
        // cycles
        self.peek8(address)
    }
    fn absolute_indexed_read(&mut self, indexed: u8) -> u8 {
        let address = self.fetch16() + indexed as u16;
        self.peek8(address)
    }
}
