/*
Supported mappers:
... : bad mappers
--- : potentially bad mappers
=== : not supported by both Nestopia & FCEUX
??? : No known roms
-----------------------------------------------------------------
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10| 11| 12| 13| 14| 15|
| 16| 17| 18| 19|...| 21| 22| 23| 24| 25| 26| 27| 28| 29| 30| 31|
| 32| 33| 34| 35| 36| 37| 38| 39| 40| 41| 42| 43| 44| 45| 46| 47|
| 48| 49| 50| 51| 52| 53| 54|???| 56| 57| 58| 59| 60| 61| 62| 63|
| 64| 65| 66| 67| 68| 69| 70| 71| 72| 73| 74| 75| 76| 77| 78| 79|
| 80|===| 82| 83|===| 85| 86| 87| 88| 89| 90| 91| 92| 93| 94| 95|
| 96| 97|===| 99|...|101|===|103|104|105|106|107|108|===|===|111|
|112|113|114|115|116|117|118|119|120|121|===|123|===|125|126|===|
|===|===|===|===|132|133|134|===|136|137|138|139|140|141|142|143|
|144|145|146|147|148|149|150|151|152|153|154|155|156|157|158|159|
|---|===|162|163|164|165|166|167|168|===|170|171|172|173|174|175|
|176|177|178|179|180|---|182|183|184|185|186|187|188|189|190|191|
|192|193|194|195|196|197|198|199|200|201|202|203|204|205|206|207|
|208|209|210|211|212|213|214|215|216|217|218|219|220|221|222|???|
|224|225|226|227|228|229|230|231|232|233|234|235|236|===|238|===|
|240|241|242|243|244|245|246|===|===|249|250|===|252|253|254|255|
-----------------------------------------------------------------
*/
use crate::*;
use log::{debug, info};

mod nrom;

pub use nrom::*;

bitflags! {
    pub struct MemoryAccessType: i8 {
        const Unspecified = -1;
        const NoAccess = 0x00;
        const Read = 0x01;
        const Write = 0x02;
        const ReadWrite = 0x03;
    }
}

impl Default for MemoryAccessType {
    fn default() -> Self {
        MemoryAccessType::Unspecified
    }
}

pub enum PrgMemoryType {
    PrgRom,
    SaveRam,
    WorkRam,
}

pub enum ChrMemoryType {
    Default,
    ChrRom,
    ChrRam,
    NametableRam,
}

impl Default for Box<dyn Mapper> {
    fn default() -> Self {
        Box::new(Nrom::default())
    }
}

pub trait Mapper {
    fn id(&self) -> u8;

    fn base_mapper(&mut self) -> &mut BaseMapper;

    fn prg_page_size(&mut self) -> u16;

    fn chr_page_size(&mut self) -> u16;

    fn read_ram(&mut self, addr: u16) -> u8 {
        let proto = self.base_mapper();
        let hi = proto.prg_pages[(addr >> 8) as usize];
        let lo = addr & 0xFF;
        let addr = lo | hi;
        proto.prg_rom[addr as usize]
    }

    fn write_ram(&mut self, addr: u16, byte: u8) {
        let proto = self.base_mapper();
        let hi = proto.prg_pages[(addr >> 8) as usize];
        let lo = addr & 0xFF;
        let addr = lo | hi;
        proto.prg_rom[addr as usize] = byte;
    }

    fn read_vram(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn write_vram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    fn select_prg_page(&mut self, slot: u16, page: u16, prg_memory_type: PrgMemoryType) {
        let start_addr = 0x8000 + slot * self.prg_page_size();
        let end_addr = start_addr + (self.prg_page_size() - 1);
        debug!("prg start_addr 0x{:04X}", start_addr);
        debug!("prg end_addr 0x{:04X}", end_addr);
        let start_slot = start_addr >> 8;
        let end_slot = end_addr >> 8;
        debug!("prg start_slot 0x{:04X}", start_slot);
        debug!("prg end_slot 0x{:04X}", end_slot);
        let mut page_addr = page * self.prg_page_size();
        let proto = self.base_mapper();
        let mask = proto.prg_mask;
        for slot in start_slot..=end_slot {
            proto.prg_pages[slot as usize] = page_addr & mask;
            page_addr += 0x100;
        }
    }
    fn select_chr_page(&mut self, slot: u16, page: u16, chr_memory_type: ChrMemoryType) {
        let start_addr = slot * self.chr_page_size();
        let end_addr = start_addr + (self.chr_page_size() - 1);
        debug!("chr start_addr {:04x}", start_addr);
        debug!("chr end_addr {:04x}", end_addr);
        let start_slot = start_addr >> 8;
        let end_slot = end_addr >> 8;
        debug!("chr start_slot 0x{:04X}", start_slot);
        debug!("chr end_slot 0x{:04X}", end_slot);
        let mut page_addr = page * self.chr_page_size();
        let proto = self.base_mapper();
        let mask = proto.chr_mask;
        for slot in start_slot..=end_slot {
            proto.chr_pages[slot as usize] = page_addr & mask;
            page_addr += 0x100;
        }
    }

    fn bus_conflicts(&self) -> bool {
        false
    }

    fn prg_size(&mut self) -> u16 {
        self.base_mapper().prg_rom.len() as u16
    }

    fn prg_page_count(&mut self) -> u16 {
        let prg_page_size = self.prg_page_size();
        let prg_size = self.prg_size();
        if prg_page_size > 0 {
            prg_size / prg_page_size
        } else {
            0
        }
    }

    fn chr_size(&mut self) -> u16 {
        self.base_mapper().chr_rom.len() as u16
    }

    fn chr_page_count(&mut self) -> u16 {
        let chr_page_size = self.chr_page_size();
        let chr_size = self.chr_size();
        if chr_page_size > 0 {
            chr_size / chr_page_size
        } else {
            0
        }
    }

    fn notify_vram_address_change(&mut self, addr: u16) {}
}

impl dyn Mapper {
    pub fn new(rom_file: &VirtualFile) -> Box<dyn Mapper> {
        let rom_data = RomData::new(rom_file);
        let mut this = match rom_data.info.mapper_id {
            0 => Box::new(Nrom::new(&rom_data)),
            _ => panic!(),
        };
        debug!("prg_pages {:04x?}", this.base_mapper().prg_pages);
        debug!("chr_pages {:04x?}", this.base_mapper().chr_pages);
        this
    }
}

#[derive(Default)]
pub struct BaseMapper {
    nametable_ram: Vec<u8>,
    only_chr_ram: bool,
    allow_register_read: bool,
    prg_memory_access: Vec<MemoryAccessType>,
    chr_memory_access: Vec<MemoryAccessType>,
    prg_memory_offset: Vec<i32>,
    prg_memory_type: Vec<PrgMemoryType>,
    chr_memory_offset: Vec<i32>,
    chr_memory_type: Vec<ChrMemoryType>,
    original_prg_rom: Vec<u8>,
    original_chr_rom: Vec<u8>,
    chr_ram: Vec<u8>,
    save_ram: Vec<u8>,
    work_ram: Vec<u8>,
    vram_open_bus_value: i16,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub is_read_register_addr: Vec<bool>,
    pub is_write_register_addr: Vec<bool>,
    pub rom_info: RomInfo,
    pub prg_pages: Vec<u16>,
    pub chr_pages: Vec<u16>,
    pub prg_mask: u16,
    pub chr_mask: u16,
}

impl BaseMapper {
    fn force_chr_battery(&self) -> bool {
        todo!()
    }

    fn force_save_ram_size(&self) -> bool {
        todo!()
    }

    fn force_work_ram_size(&self) -> bool {
        todo!()
    }

    fn get_save_ram_page_size(&self) -> usize {
        todo!()
    }

    fn get_chr_ram_page_size(&self) -> usize {
        todo!()
    }

    fn validate_address_range(&self, start_addr: u16, end_addr: u16) -> bool {
        todo!()
    }

    fn get_work_ram_page_size(&self) -> usize {
        todo!()
    }

    fn register_start_address(&self) -> u16 {
        0x8000
    }

    fn register_end_address(&self) -> u16 {
        0xFFFF
    }

    fn get_dip_switch_count(&self) -> usize {
        todo!()
    }

    fn write_register(&mut self, addr: usize, byte: u8) {
        todo!()
    }

    fn read_register(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn set_cpu_memory_mapping(
        start_addr: u16,
        end_addr: u16,
        page_number: u16,
        prg_memory_type: PrgMemoryType,
        access_type: i8,
    ) {
        todo!()
    }

    fn remove_cpu_memory_mapping(&mut self, start_addr: u16, end_addr: u16) {
        todo!()
    }

    fn set_ppu_memory_mapping(
        start_addr: u16,
        end_addr: u16,
        page_number: u16,
        chr_memory_type: ChrMemoryType,
    ) {
        todo!()
    }

    fn remove_ppu_memory_mapping(&mut self, start_addr: u16, end_addr: u16) {
        todo!()
    }

    fn has_battery(&self) -> bool {
        todo!()
    }

    fn load_battery(&mut self) {
        todo!()
    }

    fn get_power_on_byte(&self, default_value: u8) -> u8 {
        todo!()
    }

    fn get_dip_switches(&self) -> u32 {
        todo!()
    }

    fn setup_default_work_ram() {
        todo!()
    }

    fn initialize_chr_ram(&mut self, chr_ram_size: usize) {
        todo!()
    }

    fn add_register_range(
        &mut self,
        start_addr: u16,
        end_addr: u16,
        memory_operation: MemoryOperation,
    ) {
        todo!()
    }

    fn remove_register_range(start_addr: u16, end_addr: u16, memory_operation: MemoryOperation) {
        todo!()
    }

    fn restore_prg_chr_state(&mut self) {
        todo!()
    }

    fn get_nametable(&self, nametable_index: u8) -> &Vec<u8> {
        todo!()
    }

    fn set_nametable(&mut self, index: usize, nametable_index: u8) {
        todo!()
    }

    fn set_mirroring_type(&mut self, mirroring_type: MirroringType) {
        todo!()
    }

    fn get_mirroring_type(&self) -> MirroringType {
        todo!()
    }

    pub fn new(rom_data: &RomData) -> Self {
        let mut this = BaseMapper::default();
        this.rom_info = rom_data.info.clone();
        this.is_read_register_addr = vec![false; RAM_SIZE];
        this.is_write_register_addr = vec![false; RAM_SIZE];
        this.prg_rom = rom_data.prg_rom.clone();
        this.chr_rom = rom_data.chr_rom.clone();
        this.prg_mask = (this.prg_rom.len() - 1) as u16;
        this.chr_mask = (this.chr_rom.len() - 1) as u16;
        this.original_prg_rom = rom_data.prg_rom.clone();
        this.original_chr_rom = rom_data.chr_rom.clone();
        info!(
            "prg_rom 0x{:04X} 0x{:04X}",
            this.prg_rom.len(),
            this.prg_mask
        );
        info!(
            "chr_rom 0x{:04X} 0x{:04X}",
            this.chr_rom.len(),
            this.chr_mask
        );
        this.prg_pages = vec![0; 0x100];
        this.chr_pages = vec![0; 0x100];
        this
    }

    pub fn reset(&mut self, soft_reset: bool) {
        todo!()
    }

    pub fn get_available_features(&self) -> ConsoleFeatures {
        todo!()
    }

    pub fn set_nes_model(&mut self, model: NesModel) {
        todo!()
    }

    pub fn process_cpu_clock(console: &mut Console) {}

    pub fn notify_vram_address_change(addr: u16) {
        todo!()
    }

    pub fn save_battery(&self) {
        todo!()
    }

    pub fn apply_samples(&mut self, buffer: Vec<i16>, volume: f64) {
        todo!()
    }

    pub fn read_ram(&mut self, addr: usize) {
        todo!()
    }

    pub fn write_ram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn write_prg_ram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn read_vram(&mut self, addr: u16) -> u8 {
        todo!()
    }

    pub fn write_vram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn copy_chr_tile(&self, addr: u32, dest: &mut Vec<u8>) {
        todo!()
    }

    pub fn has_chr_ram(&self) -> bool {
        todo!()
    }

    pub fn has_chr_rom(&self) -> bool {
        todo!()
    }

    pub fn get_cartridge_state(&self) -> CartridgeState {
        todo!()
    }

    pub fn get_memory_value(&mut self, memory_type: DebugMemoryType, addr: u16) -> u8 {
        todo!()
    }

    pub fn set_memory_value(&mut self, memory_type: DebugMemoryType, addr: u16, byte: u8) {
        todo!()
    }

    pub fn get_memory_size(&self, memory_type: DebugMemoryType) -> usize {
        todo!()
    }

    pub fn copy_memory(&self, debug_memory: DebugMemoryType, buffer: &mut Vec<u8>) {
        todo!()
    }

    pub fn get_cpu_absolute_address_and_type(&self, relative_addr: u32) -> CpuAddressInfo {
        todo!()
    }

    pub fn get_ppu_absolute_address_and_type(&self, relative_addr: u32) -> PpuAddressInfo {
        todo!()
    }

    pub fn to_absolute_address(&self, addr: u16) -> usize {
        todo!()
    }
}
