use crate::*;
use log::info;

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
