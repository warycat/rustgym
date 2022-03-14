use crate::*;

#[derive(Debug, Default)]
pub struct CpuState {
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    ps: u8,
    irq_flag: u32,
    cycle_count: u64,
    nmi_flag: bool,
    debug_pc: u16,
    prev_debug_pc: u16,
}

impl CpuState {
    pub fn new() -> Self {
        CpuState {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            ps: 0,
            irq_flag: 0,
            cycle_count: 0,
            nmi_flag: false,
            debug_pc: 0,
            prev_debug_pc: 0,
        }
    }
}

#[derive(Default)]
pub struct CartridgeState {
    prg_rom_size: usize,
    chr_rom_size: usize,
    chr_ram_size: usize,
    prg_page_count: usize,
    prg_page_size: usize,
    prg_memory_offset: Vec<i32>,
    prg_memory_access: Vec<MemoryAccessType>,
    chr_page_count: usize,
    chr_page_size: usize,
    chr_ram_page_size: usize,
    chr_memory_offset: Vec<i32>,
    chr_type: Vec<ChrMemoryType>,
    chr_memory_access: Vec<MemoryAccessType>,
    work_ram_page_size: usize,
    save_ram_page_size: usize,
    mirroring_type: MirroringType,
    has_battery: bool,
}

#[derive(Default)]
pub struct PpuState {
    pub control_flags: PpuControlFlags,
    pub mask_flags: PpuMaskFlags,
    pub status_flags: PpuStatusFlags,
    pub sprite_ram_addr: u8,
    pub video_ram_addr: u16,
    pub x_scroll: u8,
    pub tmp_video_ram_addr: u16,
    pub write_toggle: bool,
    pub high_bit_shift: u16,
    pub low_bit_shift: u16,
    pub scanline: i32,
    pub cycle: u32,
    pub frame_count: u32,
    pub nmi_scanline: i32,
    pub safe_oam_scanline: u32,
    pub bus_address: u16,
    pub memory_read_buffer: u8,
}

#[derive(Default)]
pub struct ApuState {}

#[derive(Default)]
pub struct DebugState {}
