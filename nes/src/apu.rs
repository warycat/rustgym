use crate::*;
use log::info;

#[derive(Debug, Default)]
pub struct Apu {}

impl MemoryHandler for Apu {
    fn read_byte(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, addr: u16, byte: u8) {}
}

impl Apu {
    pub fn process_cpu_clock(&mut self) {
        // todo!()
    }
}
