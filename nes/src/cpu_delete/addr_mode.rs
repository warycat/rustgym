use crate::*;

#[derive(Debug)]
pub enum AddrMode {
    Non,
    Acc,
    Imp,
    Imm,
    Rel,
    Zpg,
    Abs,
    ZpX,
    ZpY,
    Ind,
    AbX,
    AbsY,
    IndX,
    IndY,
    InYW,
    AbXW,
    AbYW,
}
use AddrMode::*;

impl Default for AddrMode {
    fn default() -> Self {
        Non
    }
}

impl AddrMode {
    fn byte_size(&self) -> usize {
        match self {
            Imp => 0,
            Imm => 1,
            Zpg => 1,
            ZpX => 1,
            ZpY => 1,
            Abs => 2,
            AbX => 1,
            AbsY => 1,
            Ind => 2,
            IndX => 1,
            IndY => 1,
            Rel => 2,
            _ => 0,
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

pub static ADDR_MODE: [AddrMode; 0x100] = [
    Imp, IndX, Non, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Abs, Abs, Abs, Abs, //
    Rel, IndY, Non, InYW, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbYW, AbX, AbX, AbXW, AbXW, //
    Abs, IndX, Non, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Abs, Abs, Abs, Abs, //
    Rel, IndY, Non, InYW, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbYW, AbX, AbX, AbXW, AbXW, //
    Imp, IndX, Non, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Abs, Abs, Abs, Abs, //
    Rel, IndY, Non, InYW, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbYW, AbX, AbX, AbXW, AbXW, //
    Imp, IndX, Non, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Ind, Abs, Abs, Abs, //
    Rel, IndY, Non, InYW, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbYW, AbX, AbX, AbXW, AbXW, //
    Imm, IndX, Imm, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, //
    Rel, InYW, Non, InYW, ZpX, ZpX, ZpY, ZpY, Imp, AbYW, Imp, AbYW, AbXW, AbXW, AbYW, AbYW, //
    Imm, IndX, Imm, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, //
    Rel, IndY, Non, IndY, ZpX, ZpX, ZpY, ZpY, Imp, AbsY, Imp, AbsY, AbX, AbX, AbsY, AbsY, //
    Imm, IndX, Imm, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, //
    Rel, IndY, Non, InYW, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbYW, AbX, AbX, AbXW, AbXW, //
    Imm, IndX, Imm, IndX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, //
    Rel, IndY, Non, InYW, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbYW, AbX, AbX, AbXW, AbXW, //
];
