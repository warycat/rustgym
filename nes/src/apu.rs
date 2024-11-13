use crate::*;

#[derive(Debug, Default)]
pub struct Apu {}

impl MemoryHandler for Apu {
    fn read_byte(&mut self, _addr: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, _addr: u16, _byte: u8) {}
}

impl Apu {
    pub fn process_cpu_clock(&mut self) {
        // todo!()
    }
}
