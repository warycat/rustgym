use crate::base::*;
use std::ops::Range;

#[derive(Debug)]
pub enum HeaderVersion {
    INes,
    Other,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct NesHeader {
    bytes: [u8; 16],
}

impl NesHeader {
    fn prg_count(&self) -> usize {
        if self.bytes[4] == 0 {
            0x100
        } else {
            self.bytes[4] as usize
        }
    }

    fn chr_count(&self) -> usize {
        self.bytes[5] as usize
    }

    fn prg_start(&self) -> usize {
        self.header_size() + self.trainer_size()
    }

    fn prg_end(&self) -> usize {
        self.prg_start() + self.prg_size()
    }

    fn chr_start(&self) -> usize {
        self.prg_end()
    }

    fn chr_end(&self) -> usize {
        self.chr_start() + self.chr_size()
    }

    pub fn new(data: &[u8]) -> NesHeader {
        let n = data.len();
        assert!(n > 16);
        let mut bytes: [u8; 16] = [0; 16];
        bytes.copy_from_slice(&data[0..16]);
        assert_eq!(&bytes[0..4], b"NES\x1A");
        NesHeader { bytes }
    }

    pub fn header_version(&self) -> HeaderVersion {
        match self.bytes[7] & 0x0C {
            0x00 => HeaderVersion::INes,
            _ => HeaderVersion::Other,
        }
    }

    pub fn mapper_id(&self) -> u8 {
        match self.header_version() {
            HeaderVersion::INes => self.bytes[6] >> 4 | (self.bytes[7] & 0xF0),
            _ => panic!(),
        }
    }

    pub fn has_battery(&self) -> bool {
        self.bytes[6] & 0x02 == 0x02
    }

    pub fn has_trainer(&self) -> bool {
        self.bytes[6] & 0x04 != 0
    }

    pub fn prg_size(&self) -> usize {
        match self.header_version() {
            HeaderVersion::INes => self.prg_count() * 0x4000,
            _ => panic!(),
        }
    }

    pub fn chr_size(&self) -> usize {
        match self.header_version() {
            HeaderVersion::INes => self.chr_count() as usize * 0x2000,
            _ => panic!(),
        }
    }

    pub fn mirroring_type(&self) -> MirroringType {
        if self.bytes[6] & 0x08 != 0 {
            MirroringType::FourScreens
        } else {
            if self.bytes[6] & 0x01 != 0 {
                MirroringType::Vertical
            } else {
                MirroringType::Horizontal
            }
        }
    }

    pub fn header_size(&self) -> usize {
        16
    }

    pub fn trainer_size(&self) -> usize {
        if self.has_trainer() {
            512
        } else {
            0
        }
    }

    pub fn prg_range(&self) -> Range<usize> {
        self.prg_start()..self.prg_end()
    }

    pub fn chr_range(&self) -> Range<usize> {
        self.chr_start()..self.chr_end()
    }
}
