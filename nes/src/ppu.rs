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

#[derive(PartialEq, Eq)]
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

bitflags! {
    #[derive(Default)]
    pub struct PpuControlFlags: u8 {
        const NameTableAddr = 0x03;
        const VerticalWrite = 0x04;
        const SpritePatternTableAddr = 0x08;
        const BackgroundPatternTableAddr = 0x10;
        const LargeSprites = 0x20;
        const MasterSlaveSelect = 0x40;
        const VerticalBlank = 0x80;
    }

    #[derive(Default)]
    pub struct PpuMaskFlags: u8 {
        const Grayscale = 0x01;
        const BackgroundMask = 0x02;
        const SpritesMask = 0x04;
        const BackgroundEnabled = 0x08;
        const SpritesEnabled = 0x10;
        const IntensifyRed = 0x20;
        const IntensifyGreen = 0x40;
        const IntensifyBlue = 0x80;
    }

    #[derive(Default)]
    pub struct PpuStatusFlags: u8 {
        const OpenBus = 0x1F;
        const SpriteOverflow = 0x20;
        const Sprite0Hit = 0x40;
        const VerticalBlank = 0x80;
    }
}

#[derive(Default)]
struct PpuStatus {
    sprite_overflow: bool,
    sprite0_hit: bool,
    vertical_blank: bool,
}

#[derive(Default, Clone, Copy)]
struct SpriteInfo {
    sprite_x: u8,
    low_byte: u8,
    high_byte: u8,
    palette_offset: u8,
    tile_addr: u16,
    absolute_tile_addr: i32,
    offset_y: u8,
    horizontal_mirror: bool,
    vertical_mirror: bool,
    background_priority: bool,
}

#[derive(Default)]
pub struct Ppu {
    master_clock: u64,
    memory_read_buffer: u8,
    palette_ram: Vec<u8>,
    sprite_ram: Vec<u8>,
    sprite_ram_addr: u8,
    video_ram_addr: u16,
    tmp_video_ram_addr: u16,
    x_scroll: u8,
    write_toggle: bool,
    high_bit_shift: u16,
    low_bit_shift: u16,
    has_sprite: Vec<bool>,
    output_buffers: Vec<Vec<u16>>,
    current_output_buffer_index: usize,
    nes_model: NesModel,
    standard_v_blank_end: u16,
    standard_nmi_scanline: u16,
    vblank_end: i32,
    nmi_scanline: i32,
    pal_sprite_eval_scanline: u16,
    intensify_color_bits: u16,
    palette_ram_mask: u8,
    last_updated_pixel: i32,
    bus_address: u16,
    current_tile: SpriteInfo,
    next_tile: SpriteInfo,
    previous_tile: SpriteInfo,
    sprite_tiles: Vec<SpriteInfo>,
    sprite_count: u32,
    secondary_oam_addr: u32,
    sprite0_visible: bool,
    first_visible_sprite_addr: u8,
    last_visible_sprite_addr: u8,
    sprite_index: usize,
    open_bus: u8,
    open_bus_decay_stamp: Vec<u32>,
    ignore_vram_read: u32,
    oam_copy_buffer: u8,
    sprite_in_range: bool,
    sprite0_added: bool,
    sprite_addr_h: u8,
    sprite_addr_l: u8,
    oam_copy_done: bool,
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
    safe_oam_scanline: u32,
    pub control_flags: PpuControlFlags,
    pub mask_flags: PpuMaskFlags,
    pub status_flags: PpuStatusFlags,
    pub cycle: u32,
    pub frame_count: u32,
    pub scanline: i32,
    pub secondary_sprite_ram: Vec<u8>,
    pub settings: EmulationFlags,
}

impl Ppu {
    fn update_status_flags(console: &mut Console) {
        let ppu = &mut console.ppu;
        ppu.status_flags.set(PpuStatusFlags::VerticalBlank, false);
        if ppu.scanline == ppu.nmi_scanline && ppu.cycle == 0 {
            ppu.prevent_vbl_flag = true;
        }
    }

    fn set_control_register(&mut self, byte: u8) {
        self.control_flags = PpuControlFlags::from_bits(byte).unwrap();
    }

    fn set_mask_register(&mut self, byte: u8) {
        todo!()
    }

    fn process_tmp_addr_scroll_glitch(normal_addr: u16, value: u16, mask: u16) {
        todo!()
    }

    fn set_open_bus(&mut self, mut mask: u8, mut byte: u8) {
        if mask == 0xFF {
            self.open_bus = byte;
            for i in 0..8 {
                self.open_bus_decay_stamp[i] = self.frame_count;
            }
        } else {
            let mut open_bus = (self.open_bus as u16) << 8;
            for i in 0..8 {
                open_bus >>= 1;
                if mask & 0x01 == 0x01 {
                    if byte & 0x01 == 0x01 {
                        open_bus |= 0x80;
                    } else {
                        open_bus &= 0xFF7F;
                    }
                    self.open_bus_decay_stamp[i] = self.frame_count;
                } else if self.frame_count - self.open_bus_decay_stamp[i] > 30 {
                    open_bus &= 0xFF7F;
                }
                byte >>= 1;
                mask >>= 1;
            }
            self.open_bus = open_bus as u8;
        }
    }

    fn apply_open_bus(&mut self, mask: u8, byte: u8) -> u8 {
        self.set_open_bus(!mask, byte);
        byte | self.open_bus & mask
    }

    fn process_status_reg_open_bus(
        console: &mut Console,
        open_bus_mask: &mut u8,
        return_value: &mut u8,
    ) {
        use PpuModel::*;
        match console.emulation_settings.ppu_model {
            Ppu2C05A => {
                *open_bus_mask = 0x00;
                *return_value |= 0x1B;
            }
            Ppu2C05B => {
                *open_bus_mask = 0x00;
                *return_value |= 0x3D;
            }
            Ppu2C05C => {
                *open_bus_mask = 0x00;
                *return_value |= 0x1C;
            }
            Ppu2C05D => {
                *open_bus_mask = 0x00;
                *return_value |= 0x1B;
            }
            Ppu2C05E => {
                *open_bus_mask = 0x00;
            }
            _ => {}
        }
    }

    fn update_video_ram_addr(console: &mut Console) {
        let ppu = &mut console.ppu;
        if ppu.scanline >= 240 || !ppu.rendering_enabled {
            ppu.video_ram_addr = (ppu.video_ram_addr
                + if ppu.control_flags.contains(PpuControlFlags::VerticalWrite) {
                    0x20
                } else {
                    0x00
                })
                & 0x7FFF;
            Ppu::set_bus_address(console, console.ppu.video_ram_addr & 0x3FFF);
        } else {
            ppu.inc_horizontal_scrolling();
            ppu.inc_vertical_scrolling();
        }
    }

    fn inc_vertical_scrolling(&mut self) {
        let mut addr = self.video_ram_addr;
        if addr & 0x7000 != 0x7000 {
            addr += 0x1000;
        } else {
            addr &= !0x7000;
            let mut y = (addr & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                addr ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            addr = addr & !0x03E0 | y << 5
        }
        self.video_ram_addr = addr;
    }

    fn inc_horizontal_scrolling(&mut self) {
        let mut addr = self.video_ram_addr;
        if addr & 0x001F == 31 {
            addr = (addr & !0x001F) ^ 0x0400;
        } else {
            addr += 1;
        }
        self.video_ram_addr = addr;
    }

    fn get_name_table_addr(&self) -> u16 {
        todo!()
    }

    fn get_attribute_addr(&self) -> u16 {
        todo!()
    }

    fn process_scanline(console: &mut Console) {
        match console.ppu.cycle {
            0..=256 => {
                let ppu = &mut console.ppu;
                ppu.load_tile_info();
                if ppu.prev_rendering_enabled && ppu.cycle & 0x07 == 0 {
                    ppu.inc_horizontal_scrolling();
                    if ppu.cycle == 256 {
                        ppu.inc_horizontal_scrolling();
                    }
                }
                if ppu.scanline >= 0 {
                    Ppu::draw_pixel(console);
                    let ppu = &mut console.ppu;
                    ppu.shift_tile_registers();
                    ppu.process_sprite_evaluation();
                } else if ppu.cycle < 9 {
                    if ppu.cycle == 1 {
                        ppu.status_flags.remove(PpuStatusFlags::VerticalBlank);
                        console.cpu.nmi_flag = false;
                    }
                    if ppu.sprite_ram_addr >= 0x08
                        && ppu.rendering_enabled
                        && ppu.settings.contains(EmulationFlags::DisableOamAddrBug)
                    {
                        let addr = ppu.sprite_ram_addr & 0xF8 + (ppu.cycle - 1) as u8;
                        let byte = ppu.read_sprite_ram(addr);
                        ppu.write_sprite_ram((ppu.cycle - 1) as u8, byte);
                    }
                }
            }
            257..=320 => {
                let ppu = &mut console.ppu;
                if ppu.cycle == 257 {
                    ppu.sprite_index = 0;
                    ppu.has_sprite = vec![false; 257];
                    if ppu.prev_rendering_enabled {
                        ppu.video_ram_addr =
                            ppu.video_ram_addr & !0x041F | ppu.tmp_video_ram_addr & 0x041F;
                    }
                }
                if ppu.rendering_enabled {
                    ppu.sprite_ram_addr = 0;
                    todo!();
                }
                if ppu.scanline == -1 && ppu.cycle >= 280 && ppu.cycle <= 304 {
                    ppu.video_ram_addr =
                        ppu.video_ram_addr & !0x7BE0 | ppu.tmp_video_ram_addr & 0x7BE0;
                }
            }
            321..=336 => {
                let ppu = &mut console.ppu;
                if ppu.cycle == 321 {
                    if ppu.rendering_enabled {
                        todo!()
                    }
                    ppu.load_tile_info();
                    if ppu.scanline == -1 {
                        console.debugger.set_last_frame_ppu_scroll(
                            ppu.video_ram_addr,
                            ppu.x_scroll,
                            false,
                        );
                    }
                } else if ppu.prev_rendering_enabled && ppu.cycle == 328 || ppu.cycle == 336 {
                    ppu.load_tile_info();
                    ppu.low_bit_shift <<= 8;
                    ppu.high_bit_shift <<= 8;
                    ppu.inc_horizontal_scrolling();
                } else {
                    ppu.load_tile_info();
                }
            }
            337..=339 => {
                let ppu = &mut console.ppu;
                if ppu.rendering_enabled {
                    todo!()
                }
                if ppu.scanline == -1
                    && ppu.cycle == 339
                    && ppu.frame_count & 0x01 == 0x01
                    && ppu.nes_model == NesModel::Ntsc
                    && console.emulation_settings.ppu_model == PpuModel::Ppu2C02
                {
                    console.ppu.cycle = 340;
                }
            }
            _ => {}
        }
    }

    fn process_sprite_evaluation(&mut self) {
        if self.rendering_enabled {
            if self.cycle < 65 {
                self.oam_copy_buffer = 0xFF;
                self.secondary_sprite_ram[((self.cycle - 1) >> 1) as usize] = 0xFF;
            } else {
                if self.cycle == 65 {
                    self.sprite0_added = false;
                    self.sprite_in_range = false;
                    self.secondary_oam_addr = 0;
                    self.overflow_bug_counter = 0;
                    self.oam_copy_done = false;
                    self.sprite_addr_h = ((self.sprite_ram_addr >> 2) & 0x3F) as u8;
                    self.sprite_addr_l = (self.sprite_ram_addr & 0x03) as u8;
                    self.first_visible_sprite_addr = self.sprite_addr_h * 4;
                    self.last_visible_sprite_addr = self.first_visible_sprite_addr;
                } else if self.cycle == 256 {
                    self.sprite0_visible = self.sprite0_added;
                    self.sprite_count = self.secondary_oam_addr >> 2;
                }
                if self.cycle & 0x01 == 0x01 {
                    self.oam_copy_buffer = self.read_sprite_ram(self.sprite_ram_addr);
                } else {
                    if self.oam_copy_done {
                        self.sprite_addr_h = (self.sprite_addr_h + 1) & 0x3F;
                        if self.secondary_oam_addr >= 0x20 {
                            self.oam_copy_buffer = self.secondary_sprite_ram
                                [(self.secondary_oam_addr & 0x1F) as usize];
                        }
                    } else {
                        if !self.sprite_in_range
                            && self.scanline >= self.oam_copy_buffer as i32
                            && self.scanline
                                < self.oam_copy_buffer as i32
                                    + if self.control_flags.contains(PpuControlFlags::LargeSprites)
                                    {
                                        16
                                    } else {
                                        8
                                    }
                        {
                            self.sprite_in_range = true;
                        }
                        if self.secondary_oam_addr < 0x20 {
                            self.secondary_sprite_ram[self.secondary_oam_addr as usize] =
                                self.oam_copy_buffer;
                            if self.sprite_in_range {
                                self.sprite_addr_l += 1;
                                self.secondary_oam_addr += 1;
                                if self.sprite_addr_h == 0 {
                                    self.sprite0_added = true;
                                }
                                if self.secondary_oam_addr & 0x03 == 0 {
                                    self.sprite_in_range = false;
                                    self.sprite_addr_l = 0;
                                    self.last_visible_sprite_addr = self.sprite_addr_h * 4;
                                    self.sprite_addr_h = (self.sprite_addr_h + 1) & 0x3F;
                                    if self.sprite_addr_h == 0 {
                                        self.oam_copy_done = true;
                                    }
                                }
                            } else {
                                self.sprite_addr_h = (self.sprite_addr_h + 1) & 0x3F;
                                if self.sprite_addr_h == 0 {
                                    self.oam_copy_done = true;
                                }
                            }
                        } else {
                            self.oam_copy_buffer = self.secondary_sprite_ram
                                [(self.secondary_oam_addr & 0x1F) as usize];
                            if self.sprite_in_range {
                                self.status_flags |= PpuStatusFlags::SpriteOverflow;
                                self.sprite_addr_l = self.sprite_addr_l + 1;
                                if self.sprite_addr_l == 4 {
                                    self.sprite_addr_h = (self.sprite_addr_h + 1) & 0x3F;
                                    self.sprite_addr_l = 0;
                                }
                                if self.overflow_bug_counter == 0 {
                                    self.overflow_bug_counter = 3;
                                } else if self.overflow_bug_counter == 0 {
                                    self.overflow_bug_counter -= 3;
                                    if self.overflow_bug_counter == 0 {
                                        self.oam_copy_done = true;
                                        self.sprite_addr_l = 0;
                                    }
                                }
                            } else {
                                self.sprite_addr_h = (self.sprite_addr_h + 1) & 0x3F;
                                self.sprite_addr_l = (self.sprite_addr_l + 1) & 0x03;
                                if self.sprite_addr_h == 0 {
                                    self.oam_copy_done = true;
                                }
                            }
                        }
                    }
                    self.sprite_ram_addr = (self.sprite_addr_l & 0x03) | self.sprite_addr_h << 2;
                }
            }
        }
    }

    fn begin_v_blank(&mut self) {
        todo!()
    }

    fn trigger_nmi(&mut self) {
        todo!()
    }

    fn load_tile_info(&mut self) {
        if self.rendering_enabled {
            match self.cycle & 0x07 {
                1 => {
                    self.previous_tile = self.current_tile;
                    self.current_tile = self.next_tile;
                    // self.low_bit_shift |= self.next_tile.low_byte;
                    // self.high_bit_shift |= self.next_tile.high_byte;
                }
                _ => {}
            }
        }
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
        self.low_bit_shift <<= 1;
        self.high_bit_shift <<= 1;
    }

    fn read_sprite_ram(&mut self, addr: u8) -> u8 {
        todo!()
    }

    fn write_sprite_ram(&mut self, addr: u8, byte: u8) {
        todo!()
    }

    fn set_oam_corruption(&mut self) {
        todo!()
    }

    fn process_oam_corruption(console: &mut Console) {
        if !console
            .ppu
            .settings
            .contains(EmulationFlags::EnablePpuOamRowCorruption)
        {
            return;
        }
        let ppu = &mut console.ppu;
        for i in 0..32 {
            if ppu.corrupt_oam_row[i] {
                if i > 0 {
                    ppu.sprite_ram.copy_within(0..8, i * 8);
                }
                ppu.corrupt_oam_row[i] = false;
            }
        }
    }

    fn update_minimum_draw_cycles(&mut self) {
        self.minimum_draw_bg_cycle = if self.mask_flags.contains(PpuMaskFlags::BackgroundEnabled) {
            if self.mask_flags.contains(PpuMaskFlags::BackgroundMask)
                || self
                    .settings
                    .contains(EmulationFlags::ForceBackgroundFirstColumn)
            {
                0
            } else {
                8
            }
        } else {
            300
        };
    }

    fn get_pixel_color(console: &mut Console) -> u8 {
        let ppu = &mut console.ppu;
        let emulation_settings = &mut console.emulation_settings;
        let offset = ppu.x_scroll;
        let mut background_color = 0;
        let mut sprite_bg_color = 0;
        if ppu.cycle > ppu.minimum_draw_bg_cycle {
            let lo = (((ppu.low_bit_shift << offset) & 0x8000) >> 15) as u8;
            let hi = (((ppu.high_bit_shift << offset) & 0x8000) >> 14) as u8;
            sprite_bg_color = lo | hi;
            if emulation_settings.background_enabled {
                background_color = sprite_bg_color;
            }
        }

        if ppu.has_sprite[ppu.cycle as usize] && ppu.cycle > ppu.minimum_draw_sprite_cycle {
            for i in 0..ppu.sprite_count as usize {
                let shift = ppu.cycle - ppu.sprite_tiles[i].sprite_x as u32 - 1;
                if shift >= 0 && shift < 8 {
                    let sprite_color = if ppu.sprite_tiles[i].horizontal_mirror {
                        (ppu.sprite_tiles[i].low_byte >> shift) & 0x01
                            | ((ppu.sprite_tiles[i].high_byte >> shift) & 0x01) << 1
                    } else {
                        (ppu.sprite_tiles[i].low_byte << shift) & 0x80
                            | ((ppu.sprite_tiles[i].high_byte << shift) & 0x80) >> 6
                    };
                    if sprite_color != 0 {
                        if i == 0
                            && sprite_bg_color != 0
                            && ppu.sprite0_visible
                            && ppu.cycle != 256
                            && ppu.mask_flags.contains(PpuMaskFlags::BackgroundEnabled)
                            && !ppu.status_flags.contains(PpuStatusFlags::Sprite0Hit)
                            && ppu.cycle > ppu.minimum_draw_sprite_standard_cycle
                        {
                            ppu.status_flags |= PpuStatusFlags::Sprite0Hit;
                        }
                    }
                    if console.emulation_settings.sprites_enabled
                        && (background_color == 0 || !ppu.sprite_tiles[i].background_priority)
                    {
                        return ppu.sprite_tiles[i].palette_offset + sprite_color;
                    }
                    break;
                }
            }
        }

        let tile = if offset as u32 + ((ppu.cycle - 1) & 0x07) < 8 {
            ppu.previous_tile
        } else {
            ppu.current_tile
        };
        tile.palette_offset + background_color
    }

    pub fn current_output_buffer(&mut self) -> &mut Vec<u16> {
        &mut self.output_buffers[self.current_output_buffer_index as usize]
    }

    fn draw_pixel(console: &mut Console) {
        let ppu = &mut console.ppu;
        let offset = (((ppu.scanline as u32) << 8) + ppu.cycle - 1) as usize;
        if ppu.rendering_enabled || ppu.video_ram_addr & 0x3F00 != 0x3F00 {
            let color = Ppu::get_pixel_color(console);
            let ppu = &mut console.ppu;
            let color = if color & 0x03 != 0 { color as usize } else { 0 };
            ppu.current_output_buffer()[offset] = ppu.palette_ram[color] as u16;
        } else {
            let color = (ppu.video_ram_addr & 0x1F) as usize;
            ppu.current_output_buffer()[offset] = ppu.palette_ram[color] as u16;
        }
    }

    fn send_frame(console: &mut Console) {
        VideoDecoder::update_frame_sync(console);
        console.ppu.enable_oam_decay = console
            .ppu
            .settings
            .contains(EmulationFlags::EnableOamDecay);
    }

    fn update_state(&mut self) {
        todo!()
    }

    fn update_apu_status(&mut self) {
        todo!()
    }

    fn set_bus_address(console: &mut Console, addr: u16) {
        console.ppu.bus_address = addr;
        console.mapper.notify_vram_address_change(addr);
    }

    fn read_vram(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn write_vram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn new(settings: EmulationFlags) -> Self {
        let mut this = Ppu::default();
        this.settings = settings;
        this.master_clock = 0;
        this.output_buffers = vec![vec![0; 256 * 240]; 2];
        this.open_bus_decay_stamp = vec![0; 8];
        this.current_output_buffer_index = 0;
        this.palette_ram = PALETTE_RAM_BOOT_VALUES.to_vec();
        this.sprite_ram = vec![0; 0x100];
        this.secondary_sprite_ram = vec![0; 20];
        this.nmi_scanline = 241;
        this.pal_sprite_eval_scanline = 241 + 24;
        this.vblank_end = 260;
        this.standard_nmi_scanline = 241;
        this.standard_v_blank_end = 260;
        this.sprite_tiles = vec![SpriteInfo::default(); 64];
        this
    }

    pub fn reset(console: &mut Console) {
        let ppu = &mut console.ppu;
        ppu.master_clock = 0;
        ppu.prevent_vbl_flag = false;
        ppu.need_state_update = false;
        ppu.rendering_enabled = false;
        ppu.ignore_vram_read = 0;
        ppu.open_bus = 0;
        ppu.open_bus_decay_stamp = vec![0; 8];
        ppu.control_flags = PpuControlFlags::default();
        ppu.mask_flags = PpuMaskFlags::default();
        ppu.status_flags = PpuStatusFlags::default();
        ppu.previous_tile = SpriteInfo::default();
        ppu.current_tile = SpriteInfo::default();
        ppu.next_tile = SpriteInfo::default();
        ppu.bus_address = 0;
        ppu.intensify_color_bits = 0;
        ppu.palette_ram_mask = 0x3F;
        ppu.last_updated_pixel = -1;
        ppu.oam_copy_buffer = 0;
        ppu.sprite_in_range = false;
        ppu.sprite0_added = false;
        ppu.sprite_addr_h = 0;
        ppu.sprite_addr_l = 0;
        ppu.oam_copy_done = false;
        ppu.rendering_enabled = false;
        ppu.prev_rendering_enabled = false;
        ppu.has_sprite = vec![false; 257];
        ppu.sprite_count = 0;
        ppu.secondary_oam_addr = 0;
        ppu.sprite0_added = false;
        ppu.sprite_index = 0;
        ppu.scanline = -1;
        ppu.cycle = 340;
        ppu.frame_count = 1;
        ppu.memory_read_buffer = 0;
        ppu.overflow_bug_counter = 0;
        ppu.update_vram_addr_delay = 0;
        ppu.update_vram_addr = 0;
        ppu.oam_decay_cycles = vec![0; 0x40];
        ppu.update_minimum_draw_cycles();
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
        let control_flags = self.control_flags;
        let mask_flags = self.mask_flags;
        let status_flags = self.status_flags;
        let sprite_ram_addr = self.sprite_ram_addr;
        let video_ram_addr = self.video_ram_addr;
        let x_scroll = self.x_scroll;
        let tmp_video_ram_addr = self.tmp_video_ram_addr;
        let write_toggle = self.write_toggle;
        let high_bit_shift = self.high_bit_shift;
        let low_bit_shift = self.low_bit_shift;
        let scanline = self.scanline;
        let cycle = self.cycle;
        let frame_count = self.frame_count;
        let nmi_scanline = self.nmi_scanline;
        let safe_oam_scanline = self.safe_oam_scanline;
        let bus_address = self.bus_address;
        let memory_read_buffer = self.memory_read_buffer;

        PpuState {
            control_flags,
            mask_flags,
            status_flags,
            sprite_ram_addr,
            video_ram_addr,
            x_scroll,
            tmp_video_ram_addr,
            write_toggle,
            high_bit_shift,
            low_bit_shift,
            scanline,
            cycle,
            frame_count,
            nmi_scanline,
            safe_oam_scanline,
            bus_address,
            memory_read_buffer,
        }
    }

    pub fn read_palette_ram(&mut self, addr: u16) -> u8 {
        todo!()
    }

    pub fn write_palette_ram(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn get_overclock_rate(&self) -> f64 {
        1.0
    }

    pub fn exec(console: &mut Console) {
        let ppu = &mut console.ppu;
        if ppu.cycle > 339 {
            ppu.cycle = 0;
            ppu.scanline += 1;
            if ppu.scanline > ppu.vblank_end {
                ppu.last_updated_pixel = -1;
                ppu.scanline = -1;
                ppu.sprite_count = 0;
                if ppu.rendering_enabled {
                    Ppu::process_oam_corruption(console);
                }
                let ppu = &mut console.ppu;
                ppu.update_minimum_draw_cycles();
            }
            let ppu = &mut console.ppu;
            if ppu.scanline == console.emulation_settings.input_poll_scanline {
                console.control_manager.update_input_state();
            }

            if ppu.scanline == -1 {
                console
                    .ppu
                    .status_flags
                    .remove(PpuStatusFlags::SpriteOverflow);
                let ppu = &mut console.ppu;
                ppu.status_flags.remove(PpuStatusFlags::Sprite0Hit);
                ppu.current_output_buffer_index += 1;
                ppu.current_output_buffer_index %= 2;
            } else if ppu.scanline == 240 {
                Ppu::set_bus_address(console, console.ppu.video_ram_addr);
                Ppu::send_frame(console);
                console.ppu.frame_count += 1;
            }
        } else {
            ppu.cycle += 1;
            if ppu.scanline < 240 {
                Ppu::process_scanline(console);
            } else if ppu.cycle == 1 && ppu.scanline == ppu.nmi_scanline {
                if !ppu.prevent_vbl_flag {
                    console
                        .ppu
                        .status_flags
                        .insert(PpuStatusFlags::VerticalBlank);
                    //ppu.begin_v_blank();
                }
                let ppu = &mut console.ppu;
                ppu.prevent_vbl_flag = false;
            }
        }
        let ppu = &mut console.ppu;
        if ppu.need_state_update {
            ppu.update_state();
        }
    }

    pub fn run(console: &mut Console, run_to: u64) {
        while console.ppu.master_clock + PPU_DIVIDER <= run_to {
            Ppu::exec(console);
            console.ppu.master_clock += PPU_DIVIDER;
        }
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

    pub fn read_byte(console: &mut Console, addr: u16) -> u8 {
        let mut open_bus_mask = 0xFF;
        let mut return_value = 0;
        let ppu_register = PpuRegister::from(addr);
        use PpuRegister::*;
        match ppu_register {
            Status => {
                let ppu = &mut console.ppu;
                ppu.write_toggle = false;
                Ppu::update_status_flags(console);
                let ppu = &mut console.ppu;
                return_value = ppu.status_flags.bits();
                open_bus_mask = 0x1F;
                Ppu::process_status_reg_open_bus(console, &mut open_bus_mask, &mut return_value);
            }
            SpriteData => {
                if console
                    .ppu
                    .settings
                    .contains(EmulationFlags::DisablePpu2004Reads)
                {
                    let ppu = &mut console.ppu;
                    if ppu.scanline <= 239 && ppu.rendering_enabled {
                        if ppu.cycle >= 257 && ppu.cycle <= 320 {
                            let x = (ppu.cycle - 257) % 8;
                            let step = if x > 3 { 3 } else { x };
                            ppu.secondary_oam_addr = (ppu.cycle - 257) / 8 * 4 + step;
                            ppu.oam_copy_buffer =
                                ppu.secondary_sprite_ram[ppu.secondary_oam_addr as usize];
                        }
                        return_value = ppu.oam_copy_buffer;
                    } else {
                        return_value = ppu.read_sprite_ram(ppu.sprite_ram_addr);
                    }
                }
                open_bus_mask = 0x00;
            }
            VideoMemoryData => {
                let ppu = &mut console.ppu;
                if ppu.ignore_vram_read > 0 {
                    open_bus_mask = 0xFF;
                } else {
                    return_value = ppu.memory_read_buffer;
                    ppu.memory_read_buffer = ppu.read_vram(ppu.bus_address & 0x3FFF);
                    if ppu.bus_address & 0x3FFF >= 0x3F00
                        && !console
                            .ppu
                            .settings
                            .contains(EmulationFlags::DisablePaletteRead)
                    {
                        let ppu = &mut console.ppu;
                        return_value = ppu.read_palette_ram(ppu.bus_address) | ppu.open_bus & 0xC0;
                        open_bus_mask = 0xC0;
                    } else {
                        open_bus_mask = 0x00;
                    }
                    Ppu::update_video_ram_addr(console);
                    let ppu = &mut console.ppu;
                    ppu.ignore_vram_read = 6;
                    ppu.need_state_update = true;
                }
            }
            _ => {}
        }
        console.ppu.apply_open_bus(open_bus_mask, return_value)
    }

    pub fn write_byte(console: &mut Console, addr: u16, byte: u8) {
        let ppu = &mut console.ppu;
        if addr != 0x4014 {
            ppu.set_open_bus(0xFF, byte);
        }
        let ppu_register = PpuRegister::from(addr);
        use PpuRegister::*;
        match ppu_register {
            Control => {
                ppu.set_control_register(byte);
            }
            Mask => {
                todo!();
            }
            SpriteAddr => {
                todo!();
            }
            SpriteData => {
                todo!();
            }
            ScrollOffsets => {
                todo!();
            }
            VideoMemoryAddr => {
                todo!();
            }
            VideoMemoryData => {
                todo!();
            }
            SpriteDMA => {
                todo!();
            }
            _ => {}
        }
    }
}

#[test]
fn test() {
    logger_init();
    let rom = VirtualFile::new("Nes Test", NES_TEST);
    let mut console = Console::new(&rom, Box::new(BaseRenderer::default()));
    assert_eq!(console.mapper.id(), 0);
    console.run_frame();
    assert_eq!(console.ppu.frame_count, 2);
    console.run_frame();
    assert_eq!(console.ppu.frame_count, 3);
}
