use crate::base::*;
use crate::hash_info::HashInfo;
use crate::nes_header::NesHeader;

struct RomInfo {
    rom_name: String,
    file_name: String,
    file_prg_offset: usize,
    mapper_id: u8,
    has_chr_ram: bool,
    has_battery: bool,
    has_trainer: bool,
    mirroring_type: MirroringType,
    bus_conflicts: BusConflictType,
    hash_info: HashInfo,
    nes_header: NesHeader,
}

struct StudyBox {}

pub struct RomData {
    info: RomInfo,
    chr_ram_size: usize,
    save_chr_ram_size: usize,
    work_ram_size: usize,

    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    trainer_data: Vec<u8>,
    fds_disk_data: Vec<Vec<u8>>,
    fds_disk_headers: Vec<Vec<u8>>,
    study_box_data: StudyBox,

    raw_data: Vec<u8>,
}
