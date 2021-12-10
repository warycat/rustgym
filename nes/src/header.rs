use crate::base::*;

#[derive(Debug)]
pub struct Header {
    region: Region,
    system: System,
    ppu_model: PpuModel,
    mirroring: Mirroring,
    prg_rom: usize,
    prg_ram: usize,
    prg_nvram: usize,
    chr_rom: usize,
    chr_ram: usize,
    chr_nvram: usize,
    mapper: u16,
    sub_mapper: u8,
    version: u8,
    security: u8,
    trainer: bool,
}

impl Header {
    pub fn new(bytes: &[u8]) -> Result<(Header, NesWarning), NesError> {
        let n = bytes.len();
        let mut warning = NesWarning::OK;
        assert!(n > 16);
        let mut header: [u8; 16] = [0; 16];
        header.copy_from_slice(&bytes[0..16]);
        assert_eq!(&header[0..4], b"NES\x1A");
        let version = if header[7] & 0x0C == 0x80 { 2 } else { 0 };
        let trainer = header[6] & 0x04 != 0;
        let mirroring = if header[6] & 0x08 != 0 {
            Mirroring::FOURSCREEN
        } else {
            if header[6] & 0x01 != 0 {
                Mirroring::VERTICAL
            } else {
                Mirroring::HORIZONTAL
            }
        };
        let mut prg_rom = 0;
        let mut chr_rom = 0;
        let mut mapper = 0;

        if version == 0 {
            for i in 10..16 {
                if header[i] != 0 {
                    header[7] = 0;
                    header[8] = 0;
                    header[9] = 0;
                    warning = NesWarning::BAD_FILE_HEADER;
                    break;
                }
            }
            prg_rom = header[4] as usize * SIZE_16K;
            chr_rom = header[5] as usize * SIZE_8K;
            mapper = (header[6] >> 4 | header[7] & 0xF0) as u16;
        } else {
            panic!();
        }

        let region = Region::NTSC;
        let system = System::FAMICOM;
        let ppu_model = PpuModel::RP2C07;
        let prg_ram = 0;
        let prg_nvram = 0;
        let chr_ram = 0;
        let chr_nvram = 0;
        let sub_mapper = 0;
        let security = 0;
        let header = Header {
            region,
            system,
            ppu_model,
            mirroring,
            prg_rom,
            prg_ram,
            prg_nvram,
            chr_rom,
            chr_ram,
            chr_nvram,
            mapper,
            sub_mapper,
            version,
            security,
            trainer,
        };
        Ok((header, warning))
    }
}
