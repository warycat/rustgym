use crate::*;
use log::{debug, info};

#[derive(Debug)]
pub enum PpuRegister {
    Control = 0x00,
    Mask = 0x01,
    Status = 0x02,
    SpriteAddr = 0x03,
    SpriteData = 0x04,
    ScrollOffsets = 0x05,
    VideoMemoryAddr = 0x06,
    VideoMemoryData = 0x07,
    SpriteDMA = 0x4014,
}

impl PpuRegister {
    fn from(addr: u16) -> PpuRegister {
        if addr == 0x4014 {
            PpuRegister::SpriteDMA
        } else {
            match addr & 0x07 {
                0x00 => PpuRegister::Control,
                0x01 => PpuRegister::Mask,
                0x02 => PpuRegister::Status,
                0x03 => PpuRegister::SpriteAddr,
                0x04 => PpuRegister::SpriteData,
                0x05 => PpuRegister::ScrollOffsets,
                0x06 => PpuRegister::VideoMemoryAddr,
                0x07 => PpuRegister::VideoMemoryData,
                _ => unreachable!(),
            }
        }
    }
}

pub enum PpuModel {
    Ppu2C02,
    Ppu2C03,
    Ppu2C04A,
    Ppu2C04B,
    Ppu2C04C,
    Ppu2C04D,
    Ppu2C05A,
    Ppu2C05B,
    Ppu2C05C,
    Ppu2C05D,
    Ppu2C05E,
}

impl Default for PpuModel {
    fn default() -> Self {
        PpuModel::Ppu2C02
    }
}

#[derive(Default)]
pub struct PpuControl {
    vertical_write: bool,
    sprite_pattern_addr: u16,
    background_pattern_addr: u16,
    large_sprites: bool,
    v_blank: bool,
    grayscale: bool,
    background_mask: bool,
    sprite_mask: bool,
    background_enabled: bool,
    sprites_enabled: bool,
    intensify_red: bool,
    intensify_green: bool,
    intensify_blue: bool,
}

#[derive(Default)]
struct PpuStatus {
    sprite_overflow: bool,
    sprite0_hit: bool,
    vertical_blank: bool,
}

#[derive(Default)]
struct TileInfo {
    low_byte: u8,
    high_byte: u8,
    palette_offset: u32,
    tile_addr: u16,
    absolute_tile_addr: i32,
    offset_y: u8,
}

#[derive(Default)]
pub struct Ppu {
    master_clock: u64,
    memory_read_buffer: u8,
    palette_ram: Vec<u8>,
    sprite_ram: Vec<u8>,
    has_sprite: Vec<bool>,
    output_buffers: Vec<Vec<u16>>,
    current_output_buffer: usize,
    nes_model: NesModel,
    standard_v_blank_end: u16,
    standard_nmi_scanline: u16,
    vblank_end: u16,
    nmi_scanline: u16,
    status: PpuStatus,
    intensify_color_bits: u16,
    palette_ram_mask: u8,
    last_updated_pixel: i32,
    ppu_bus_address: u16,
    current_tile: TileInfo,
    next_tile: TileInfo,
    previous_tile: TileInfo,
    sprite_count: u32,
    secondary_oam_addr: u32,
    sprite0_visible: bool,
    first_visible_sprite_addr: u8,
    last_visible_sprite_addr: u8,
    sprite_index: usize,
    open_bus: u8,
    open_bus_decay_stamp: Vec<i32>,
    ignore_vram_read: u32,
    oam_copy_buffer: u8,
    overflow_bug_counter: u8,
    need_state_update: bool,
    rendering_enabled: bool,
    prev_rendering_enabled: bool,
    prevent_vbl_flag: bool,
    update_vram_addr: u16,
    update_vram_addr_delay: u8,
    minimum_draw_bg_cycle: u32,
    minimum_draw_sprite_cycle: u32,
    minimum_draw_sprite_standard_cycle: u32,
    oam_decay_cycles: Vec<u64>,
    enable_oam_decay: bool,
    corrupt_oam_row: Vec<bool>,
    pub control: PpuControl,
    pub cycle: u32,
    pub frame_count: u32,
    pub scanline: i32,
    pub secondary_sprite_ram: Vec<u8>,
}

impl Ppu {
    fn update_status(&mut self) {
        todo!()
    }

    fn set_control_register(&mut self, byte: u8) {
        todo!()
    }

    fn set_mask_register(&mut self, byte: u8) {
        todo!()
    }

    fn is_rendering_enabled(&self) -> bool {
        todo!()
    }

    fn process_tmp_addr_scroll_glitch(normal_addr: u16, value: u16, mask: u16) {
        todo!()
    }

    fn set_open_bus(&mut self, mask: u8, byte: u8) {
        todo!()
    }

    fn apply_open_bus(&mut self, mask: u8, byte: u8) -> u8 {
        todo!()
    }

    fn process_status_reg_open_bus(&mut self, open_bus_mask: u8, return_value: u8) {
        todo!()
    }

    fn update_video_ram_addr(&mut self) {
        todo!()
    }

    fn inc_vertical_scrolling(&mut self) {
        todo!()
    }

    fn inc_horizontal_scrolling(&mut self) {
        todo!()
    }

    fn get_name_table_addr(&self) -> u16 {
        todo!()
    }

    fn get_attribute_addr(&self) -> u16 {
        todo!()
    }

    fn process_scanline(&mut self) {
        todo!()
    }

    fn process_sprite_evaluation(&mut self) {
        todo!()
    }

    fn begin_v_blank(&mut self) {
        todo!()
    }

    fn trigger_nmi(&mut self) {
        todo!()
    }

    fn load_tile_info(&mut self) {
        todo!()
    }

    fn load_sprite(
        &mut self,
        sprite_y: u8,
        tile_index: u8,
        attributes: u8,
        sprite_x: u8,
        extra_sprite: bool,
    ) {
        todo!()
    }

    fn load_sprite_tile_info(&mut self) {
        todo!()
    }

    fn load_extra_sprites(&mut self) {
        todo!()
    }

    fn shift_tile_registers(&mut self) {
        todo!()
    }

    fn read_sprite_ram(&mut self, addr: u8) {
        todo!()
    }

    fn write_sprite_ram(&mut self, addr: u8) {
        todo!()
    }

    fn set_oam_corruption(&mut self) {
        todo!()
    }

    fn update_minimum_draw_cycles(&mut self) {
        todo!()
    }

    fn get_pixel_color(&self) -> u8 {
        todo!()
    }

    fn draw_pixel(&mut self) {
        todo!()
    }

    fn update_grayscale_and_intensify_bits(&mut self) {
        todo!()
    }

    fn send_frame(&mut self) {
        todo!()
    }

    fn update_state(&mut self) {
        todo!()
    }

    fn update_apu_status(&mut self) {
        todo!()
    }

    fn set_bus_address(&mut self, addr: u16) {
        todo!()
    }

    fn read_vram(&mut self, addr: u16) {
        todo!()
    }

    fn write_vram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn new() -> Self {
        let mut this = Ppu::default();
        this.master_clock = 0;
        this.output_buffers = vec![vec![256 * 240]; 2];
        info!("{:?}", this.output_buffers);
        this
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn debug_send_frame(&mut self) {
        todo!()
    }

    pub fn get_screen_buffer(&self, previous_buffer: bool) -> &[u16] {
        todo!()
    }

    pub fn debug_copy_output_buffer(&mut self, target: &[u16]) {
        todo!()
    }

    pub fn get_state(&self) -> PpuState {
        todo!()
    }

    pub fn read_palette_ram(&mut self, addr: u16) {
        todo!()
    }

    pub fn write_palette_ram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn set_nes_model(&mut self, nes_model: NesModel) {
        todo!()
    }

    pub fn get_overclock_rate(&self) -> f64 {
        todo!()
    }

    pub fn exec(&mut self) {
        if self.cycle > 339 {
            self.cycle = 0;
            self.scanline += 1;
            // if self.scanline > self.vblank_end {
            //     self.last_updated_pixel = -1;

            // }
        }
    }

    pub fn run(&mut self, run_to: u64) {
        while self.master_clock + self.master_clock <= run_to {
            self.exec();
            self.master_clock += PPU_DIVIDER;
        }
    }

    pub fn get_frame_count(&self) -> u32 {
        todo!()
    }

    pub fn get_frame_cycle(&self) -> u32 {
        todo!()
    }

    pub fn get_pixel_brightness(&self, x: u8, y: u8) -> u32 {
        todo!()
    }

    pub fn get_current_bg_color(&self) -> u16 {
        todo!()
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> u16 {
        todo!()
    }
}

impl MemoryHandler for Ppu {
    fn read_byte(&mut self, addr: u16) -> u8 {
        let open_bus_mask = 0xFF;
        let return_value = 0;
        let ppu_register = PpuRegister::from(addr);
        use PpuRegister::*;
        match ppu_register {
            Status => {
                debug!("{:?}", ppu_register);
            }
            SpriteData => {
                debug!("{:?}", ppu_register);
            }
            VideoMemoryData => {
                debug!("{:?}", ppu_register);
            }
            _ => {}
        }
        return_value
    }

    fn write_byte(&mut self, addr: u16, byte: u8) {
        info!("{} {}", addr, byte);
    }
}
