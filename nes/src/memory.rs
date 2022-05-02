use crate::*;
use log::info;
use std::cell::RefCell;

pub trait MemoryHandler {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, byte: u8);
    fn read_word(&mut self, addr: u16) -> u16 {
        let b0 = self.read_byte(addr);
        let b1 = self.read_byte(addr + 1);
        b0 as u16 | (b1 as u16) << 8
    }
}

bitflags! {
    pub struct MemoryOperation: u8 {
        const Read = 0x01;
        const Write = 0x02;
        const Any = 0x03;
    }
}

impl Default for MemoryOperation {
    fn default() -> Self {
        MemoryOperation::Any
    }
}

#[derive(Default)]
pub struct OpenBus {
    last_read_value: u8,
}

impl OpenBus {
    pub fn set_open_bus(&mut self, value: u8) {
        self.last_read_value = value;
    }
    pub fn get_open_bus(&self) -> u8 {
        self.last_read_value
    }
}

impl MemoryHandler for OpenBus {
    fn read_byte(&mut self, _addr: u16) -> u8 {
        self.get_open_bus()
    }
    fn write_byte(&mut self, _addr: u16, _byte: u8) {}
}

pub struct InternalRam {
    ram: Vec<u8>,
    mask: u16,
}

impl Default for InternalRam {
    fn default() -> Self {
        let ram = vec![0; INTERNAL_RAM_SIZE];
        let mask = (INTERNAL_RAM_SIZE - 1) as u16;
        InternalRam { ram, mask }
    }
}

impl MemoryHandler for InternalRam {
    fn read_byte(&mut self, addr: u16) -> u8 {
        let addr = addr & self.mask;
        self.ram[addr as usize]
    }
    fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr & self.mask;
        self.ram[addr as usize] = byte;
    }
}

#[derive(Default)]
pub struct MemoryManager {}

impl MemoryManager {
    pub fn reset(&mut self, soft_reset: bool) {
        todo!()
    }
}

impl MemoryManager {
    pub fn read_byte(console: &mut Console, addr: u16) -> u8 {
        let byte = match addr {
            0x0000..=0x1FFF => console.internal_ram.read_byte(addr),
            0x2000..=0x3FFF => Ppu::read_byte(console, addr),
            0x4000..=0x4015 => console.apu.read_byte(addr),
            0x4016..=0x4017 => ControlManager::read_byte(console, addr),
            0x4018..=0xFFFF => console.mapper.read_ram(addr),
        };
        console.open_bus.set_open_bus(byte);
        byte
    }
    pub fn write_byte(console: &mut Console, addr: u16, byte: u8) {
        match addr {
            0x0000..=0x1FFF => console.internal_ram.write_byte(addr, byte),
            0x2000..=0x3FFF => Ppu::write_byte(console, addr, byte),
            0x4000..=0x4015 => console.apu.write_byte(addr, byte),
            0x4016..=0x4017 => ControlManager::write_byte(console, addr, byte),
            0x4018..=0xFFFF => console.mapper.write_ram(addr, byte),
        }
    }
    pub fn read_word(console: &mut Console, addr: u16) -> u16 {
        let b0 = MemoryManager::read_byte(console, addr);
        let b1 = MemoryManager::read_byte(console, addr + 1);
        b0 as u16 | (b1 as u16) << 8
    }
}

#[test]
fn test() {
    let mut console = Console::default();
    MemoryManager::write_byte(&mut console, 0x4014, 10);
}
