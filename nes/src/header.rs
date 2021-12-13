use crate::base::*;
use std::ops::Range;

#[derive(Debug)]
pub enum HeaderVersion {
    V1,
    V2,
}
use HeaderVersion::*;
use Mirroring::*;

#[derive(Debug)]
pub struct Header {
    header: [u8; 16],
}

impl Header {
    pub fn new(data: &[u8]) -> Header {
        let n = data.len();
        assert!(n > 16);
        let mut header: [u8; 16] = [0; 16];
        header.copy_from_slice(&data[0..16]);
        assert_eq!(&header[0..4], b"NES\x1A");
        Header { header }
    }
    pub fn version(&self) -> HeaderVersion {
        if self.header[7] & 0x0C == 0x80 {
            V2
        } else {
            V1
        }
    }
    pub fn trainer(&self) -> bool {
        self.header[6] & 0x04 != 0
    }
    pub fn mirroring(&self) -> Mirroring {
        if self.header[6] & 0x08 != 0 {
            FOURSCREEN
        } else {
            if self.header[6] & 0x01 != 0 {
                VERTICAL
            } else {
                HORIZONTAL
            }
        }
    }
    pub fn prg_rom(&self) -> u8 {
        match self.version() {
            V1 => self.header[4],
            V2 => 0,
        }
    }
    pub fn chr_rom(&self) -> u8 {
        match self.version() {
            V1 => self.header[5],
            V2 => 0,
        }
    }
    pub fn header_size(&self) -> usize {
        16
    }
    pub fn trainer_size(&self) -> usize {
        if self.trainer() {
            512
        } else {
            0
        }
    }
    pub fn prg_rom_size(&self) -> usize {
        self.prg_rom() as usize * SIZE_16K
    }
    pub fn chr_rom_size(&self) -> usize {
        self.chr_rom() as usize * SIZE_8K
    }
    pub fn prg_rom_start(&self) -> usize {
        self.header_size() + self.trainer_size()
    }
    pub fn prg_rom_end(&self) -> usize {
        self.prg_rom_start() + self.prg_rom_size()
    }
    pub fn chr_rom_start(&self) -> usize {
        self.prg_rom_end()
    }
    pub fn chr_rom_end(&self) -> usize {
        self.chr_rom_start() + self.chr_rom_size()
    }
    pub fn prg_rom_range(&self) -> Range<usize> {
        self.prg_rom_start()..self.prg_rom_end()
    }
    pub fn mapper(&self) -> u8 {
        self.header[6] >> 4 | (self.header[7] & 0xF0)
    }
}
