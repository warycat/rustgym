use crate::cpu::Cpu;
use crate::iomap::IoMap;

pub trait Addressing {
    fn immediate(&mut self) -> u8;
    fn zero_page(&mut self) -> u16;
    fn zero_page_indexed(&mut self, indexed: u8) -> u16;
    fn absolute(&mut self) -> u16;
    fn absolute_indexed(&mut self, indexed: u8) -> u16;
    fn indirect_indexed(&mut self, indexed: u8) -> u16;
}

impl Addressing for Cpu {
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
        let address = self.fetch8() + indexed;
        address as u16
    }
    fn absolute(&mut self) -> u16 {
        let address = self.fetch16();
        address
    }
    fn absolute_indexed(&mut self, indexed: u8) -> u16 {
        let address = self.fetch16() + indexed as u16;
        address
    }
    fn indirect_indexed(&mut self, indexed: u8) -> u16 {
        let address = self.fetch8() as u16;
        let address = self.peek16(address) + indexed as u16;
        address
    }
}
