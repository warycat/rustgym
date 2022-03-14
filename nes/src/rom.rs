use crate::*;
use crc::{Crc, CRC_32_ISCSI};
use log::{debug, info};

pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

#[derive(Default, Clone, Debug)]
pub struct HashInfo {
    crc32: u32,
    pub prg_chr_md5: String,
    pub prg_crc32: u32,
    pub prg_chr_crc32: u32,
}

#[derive(Clone, Debug)]
pub enum RomFormat {
    Unknown,
    INes,
    Unif,
    Fds,
    Nsf,
    StudyBox,
}

impl Default for RomFormat {
    fn default() -> Self {
        RomFormat::Unknown
    }
}

#[derive(Default, Clone, Debug)]
pub struct RomInfo {
    filename: String,
    file_prg_offset: usize,
    has_battery: bool,
    has_trainer: bool,
    mirroring_type: MirroringType,
    nes_header: NesHeader,
    pub hash_info: HashInfo,
    pub mapper_id: u8,
    pub format: RomFormat,
}

#[derive(Default, Debug)]
struct StudyBox {}

#[derive(Default, Debug)]
pub struct RomData {
    // chr_ram_size: usize,
    // save_chr_ram_size: usize,
    // save_ram_size: usize,
    // work_ram_size: usize,
    // trainer_data: Vec<u8>,
    // fds_disk_data: Vec<Vec<u8>>,
    // fds_disk_headers: Vec<Vec<u8>>,
    // study_box_data: StudyBox,
    raw_data: Vec<u8>,
    pub info: RomInfo,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl RomData {
    pub fn new(rom_file: &VirtualFile) -> Self {
        info!("FILE {}", rom_file.filename);
        assert!(rom_file.is_valid());
        assert!(rom_file.data.len() > 16);
        assert_eq!(&rom_file.data[0..4], b"NES\x1a");
        let nes_header: NesHeader = NesHeader::new(&rom_file.data);
        let mut data_size = rom_file.data.len();
        data_size -= std::mem::size_of::<NesHeader>();
        let mut this = RomData::default();
        this.raw_data = rom_file.data.clone();
        this.info.format = RomFormat::INes;
        this.info.file_prg_offset = std::mem::size_of::<NesHeader>();
        this.info.mapper_id = nes_header.mapper_id();
        info!("Mapper {}", this.info.mapper_id);
        this.info.mirroring_type = nes_header.mirroring_type();
        this.info.has_battery = nes_header.has_battery();
        this.info.has_trainer = nes_header.has_trainer();
        this.info.nes_header = nes_header;
        assert!(!this.info.has_trainer);
        assert_eq!(data_size, nes_header.prg_size() + nes_header.chr_size());
        this.prg_rom = rom_file.data[nes_header.prg_range()].to_vec();
        this.chr_rom = rom_file.data[nes_header.chr_range()].to_vec();
        this.info.hash_info.crc32 = CASTAGNOLI.checksum(&rom_file.data);
        this.info.hash_info.prg_crc32 = CASTAGNOLI.checksum(&this.prg_rom);
        this.info.hash_info.prg_chr_crc32 = CASTAGNOLI.checksum(&rom_file.data[16..]);
        this.info.hash_info.prg_chr_md5 = format!("{:X}", md5::compute(&rom_file.data[16..]));
        debug!("FILE CRC32: {:X}", this.info.hash_info.crc32);
        debug!("PRG CRC32: {:X}", this.info.hash_info.prg_crc32);
        debug!("PRG+CHR CRC32: {:X}", this.info.hash_info.prg_chr_crc32);
        debug!("MD5: {}", this.info.hash_info.prg_chr_md5);
        this.info.filename = rom_file.filename.clone();
        this
    }
}
