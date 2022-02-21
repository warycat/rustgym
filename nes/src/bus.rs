// use crate::*;

// #[derive(Debug, Default)]
// pub struct Bus {
//     ram: Ram,
//     prg_rom: Option<PrgRom>,
//     // ppu: Ppu,
//     // apu: Apu,
// }

// impl Bus {
//     pub fn new() -> Self {
//         Bus {
//             ram: Ram::new(),
//             prg_rom: None,
//         }
//     }
//     pub fn reset(&mut self) {
//         self.ram.reset();
//     }
//     pub fn insert_cartridge(&mut self, data: &[u8]) {
//         let nes_header = NesHeader::new(data);
//         let prg_rom = PrgRom::new(nes_header.mapper_id(), &data[nes_header.prg_range()]);
//         self.prg_rom = Some(prg_rom);
//     }
// }

// impl MemoryHandler for Bus {
//     fn peek8(&mut self, address: u16) -> u8 {
//         match address {
//             0x0000..=0x1FFF => self.ram.peek8(address & 0x1FFF),
//             0x8000..=0xFFFF => self.prg_rom.as_mut().unwrap().peek8(address & 0x7FFF),
//             _ => panic!(),
//         }
//     }
//     fn poke8(&mut self, address: u16, byte: u8) {
//         match address {
//             0x0000..=0x1FFF => self.ram.poke8(address & 0x1FFF, byte),
//             0x8000..=0xFFFF => self.prg_rom.as_mut().unwrap().poke8(address & 0x7FFF, byte),
//             _ => panic!(),
//         }
//     }
// }
