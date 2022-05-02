pub type Cstring = *const u8;
pub type Wcstring = *const char;
pub type Ibool = u32;
pub type Data = u32;
pub type Address = u32;
pub type Cycle = u32;

#[derive(Debug, Default)]
pub struct Gamepad {
    pub id: String,
    pub index: u32,
    pub pressed: Vec<bool>,
    pub touched: Vec<bool>,
    pub value: Vec<f64>,
    pub axes: Vec<f64>,
    pub timestamp: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum MirroringType {
    None,
    Horizontal,
    Vertical,
    ScreenAOnly,
    ScreenBOnly,
    FourScreens,
}

impl Default for MirroringType {
    fn default() -> Self {
        MirroringType::None
    }
}

#[derive(PartialEq, Eq)]
pub enum NesModel {
    Auto,
    Ntsc,
    Pal,
    Dendy,
}

impl Default for NesModel {
    fn default() -> Self {
        NesModel::Ntsc
    }
}

pub enum Event {
    Reset,
    Nmi,
    Irq,
    StartFrame,
    EndFrame,
    CodeBreak,
    StateLoaded,
    StateSaved,
    InputPolled,
    SpriteZeroHit,
    ScriptEnded,
    BusConflict,
}

pub enum DebugEvent {
    None,
    PpuRegisterWrite,
    PpuRegisterRead,
    MapperRegisterWrite,
    MapperRegisterRead,
    ApuRegisterWrite,
    ApuRegisterRead,
    ControlRegisterWrite,
    ControlRegisterRead,
    Nmi,
    Irq,
    SpriteZeroHit,
    Breakpoint,
    DmcDmaRead,
    BgColorChange,
}

pub enum BreakSource {
    Unspecified,
    Breakpoint,
    CpuStep,
    PpuStep,
    BreakOnBrk,
    BreakOnUnofficialOpCode,
    BreakOnReset,
    BreakOnFocus,
    BreakOnUninitMemoryRead,
    BreakOnDecayedOamRead,
    BreakOnCpuCrash,
    Pause,
    BreakAfterSuspend,
    BreakOnPpu2006ScrollGlitch,
    BreakOnBusConflict,
}

impl Default for BreakSource {
    fn default() -> Self {
        BreakSource::Unspecified
    }
}

pub enum CpuAddressType {
    InternalRam,
    PrgRom,
    WorkRam,
    SaveRam,
    Register,
}

pub enum PpuAddressType {
    None,
    ChrRom,
    ChrRam,
    PaletteRam,
    NametableRam,
}

pub struct CpuAddressInfo {
    address: i32,
    address_type: CpuAddressType,
}

pub struct PpuAddressInfo {
    address: i32,
    address_type: PpuAddressType,
}

pub enum EvalResultType {
    Numeric,
    Boolean,
    Invalid,
    DivideBy0,
    OutOfScope,
}

pub enum DebugMemoryType {
    CpuMemory,
    PpuMemory,
    PaletteMemory,
    SpriteMemory,
    SecondarySpriteMemory,
    PrgRom,
    ChrRom,
    ChrRam,
    WorkRam,
    SaveRam,
    InternalRam,
    NametableRam,
}

pub const CLK_M2_MUL: u32 = 6;
pub const CLK_NTSC: u32 = 39375000 * CLK_M2_MUL;
pub const CLK_NTSC_DIV: u32 = 11;
pub const CLK_NTSC_HVSYNC: u32 = 525 * 455 * CLK_NTSC_DIV * CLK_M2_MUL / 4;
pub const CLK_PAL: u32 = 35468950 * CLK_M2_MUL;
pub const CLK_PAL_DIV: u32 = 8;
pub const CLK_PAL_HVSYNC: u32 = 625 * 1418758 / (10000 / CLK_PAL_DIV) * CLK_M2_MUL;

pub const CPU_RP2A03_CC: u32 = 12;
pub const CPU_RP2A07_CC: u32 = 16;
pub const CPU_DENDY_CC: u32 = 15;

pub const PPU_RP2C02_CC: u32 = 4;
pub const PPU_RP2C02_HACTIVE: u32 = PPU_RP2C02_CC * 256;
pub const PPU_RP2C02_HBLANK: u32 = PPU_RP2C02_CC * 85;
pub const PPU_RP2C02_HSYNC: u32 = PPU_RP2C02_HACTIVE + PPU_RP2C02_HBLANK;
pub const PPU_RP2C02_VACTIVE: u32 = 240;
pub const PPU_RP2C02_VSLEEP: u32 = 1;
pub const PPU_RP2C02_VINT: u32 = 20;
pub const PPU_RP2C02_VDUMMY: u32 = 1;
pub const PPU_RP2C02_VBLANK: u32 = PPU_RP2C02_VSLEEP + PPU_RP2C02_VINT + PPU_RP2C02_VDUMMY;
pub const PPU_RP2C02_VSYNC: u32 = PPU_RP2C02_VACTIVE + PPU_RP2C02_VBLANK;
pub const PPU_RP2C02_HVSYNCBOOT: u32 = PPU_RP2C02_VACTIVE * PPU_RP2C02_HSYNC + PPU_RP2C02_CC * 312;
pub const PPU_RP2C02_HVREGBOOT: u32 =
    (PPU_RP2C02_VACTIVE + PPU_RP2C02_VINT) * PPU_RP2C02_HSYNC + PPU_RP2C02_CC * 314;
pub const PPU_RP2C02_HVINT: u32 = PPU_RP2C02_VINT * PPU_RP2C02_HSYNC;
pub const PPU_RP2C02_HVSYNC_0: u32 = PPU_RP2C02_VSYNC * PPU_RP2C02_HSYNC;
pub const PPU_RP2C02_HVSYNC_1: u32 = PPU_RP2C02_VSYNC * PPU_RP2C02_HSYNC - PPU_RP2C02_CC;
pub const PPU_RP2C02_HVSYNC: u32 = (PPU_RP2C02_HVSYNC_0 + PPU_RP2C02_HVSYNC_1) / 2;
pub const PPU_RP2C02_FPS: u32 =
    (CLK_NTSC + CLK_NTSC_DIV * PPU_RP2C02_HVSYNC / 2) / (CLK_NTSC_DIV * PPU_RP2C02_HVSYNC);
pub const PPU_RP2C07_CC: u32 = 5;
pub const PPU_RP2C07_HACTIVE: u32 = PPU_RP2C07_CC * 256;
pub const PPU_RP2C07_HBLANK: u32 = PPU_RP2C07_CC * 85;
pub const PPU_RP2C07_HSYNC: u32 = PPU_RP2C07_HACTIVE + PPU_RP2C07_HBLANK;
pub const PPU_RP2C07_VACTIVE: u32 = 240;
pub const PPU_RP2C07_VSLEEP: u32 = 1;
pub const PPU_RP2C07_VINT: u32 = 70;
pub const PPU_RP2C07_VDUMMY: u32 = 1;
pub const PPU_RP2C07_VBLANK: u32 = PPU_RP2C07_VSLEEP + PPU_RP2C07_VINT + PPU_RP2C07_VDUMMY;
pub const PPU_RP2C07_VSYNC: u32 = PPU_RP2C07_VACTIVE + PPU_RP2C07_VBLANK;
pub const PPU_RP2C07_HVSYNCBOOT: u32 = PPU_RP2C07_VACTIVE * PPU_RP2C07_HSYNC + PPU_RP2C07_CC * 312;
pub const PPU_RP2C07_HVREGBOOT: u32 =
    (PPU_RP2C07_VACTIVE + PPU_RP2C07_VINT) * PPU_RP2C07_HSYNC + PPU_RP2C07_CC * 314;
pub const PPU_RP2C07_HVINT: u32 = PPU_RP2C07_VINT * PPU_RP2C07_HSYNC;
pub const PPU_RP2C07_HVSYNC: u32 = PPU_RP2C07_VSYNC * PPU_RP2C07_HSYNC;
pub const PPU_RP2C07_FPS: u32 =
    (CLK_PAL + CLK_PAL_DIV * PPU_RP2C07_HVSYNC / 2) / (CLK_PAL_DIV * PPU_RP2C07_HVSYNC);
pub const PPU_DENDY_CC: u32 = 5;
pub const PPU_DENDY_HACTIVE: u32 = PPU_DENDY_CC * 256;
pub const PPU_DENDY_HBLANK: u32 = PPU_DENDY_CC * 85;
pub const PPU_DENDY_HSYNC: u32 = PPU_DENDY_HACTIVE + PPU_DENDY_HBLANK;
pub const PPU_DENDY_VACTIVE: u32 = 240;
pub const PPU_DENDY_VSLEEP: u32 = 51;
pub const PPU_DENDY_VINT: u32 = 20;
pub const PPU_DENDY_VDUMMY: u32 = 1;
pub const PPU_DENDY_VBLANK: u32 = PPU_DENDY_VSLEEP + PPU_DENDY_VINT + PPU_DENDY_VDUMMY;
pub const PPU_DENDY_VSYNC: u32 = PPU_DENDY_VACTIVE + PPU_DENDY_VBLANK;
pub const PPU_DENDY_HVSYNCBOOT: u32 = PPU_DENDY_VACTIVE * PPU_DENDY_HSYNC + PPU_DENDY_CC * 312;
pub const PPU_DENDY_HVREGBOOT: u32 =
    (PPU_DENDY_VACTIVE + PPU_DENDY_VINT) * PPU_DENDY_HSYNC + PPU_DENDY_CC * 314;
pub const PPU_DENDY_HVINT: u32 = PPU_DENDY_VINT * PPU_DENDY_HSYNC;
pub const PPU_DENDY_HVSYNC: u32 = PPU_DENDY_VSYNC * PPU_DENDY_HSYNC;
pub const PPU_DENDY_FPS: u32 =
    (CLK_PAL + CLK_PAL_DIV * PPU_DENDY_HVSYNC / 2) / (CLK_PAL_DIV * PPU_DENDY_HVSYNC);

pub const RESET_CYCLES: u32 = 7;
pub const INT_CYCLES: u32 = 7;
pub const BRK_CYCLES: u32 = 7;
pub const RTI_CYCLES: u32 = 6;
pub const RTS_CYCLES: u32 = 6;
pub const PHA_CYCLES: u32 = 3;
pub const PHP_CYCLES: u32 = 3;
pub const PLA_CYCLES: u32 = 4;
pub const PLP_CYCLES: u32 = 4;
pub const JSR_CYCLES: u32 = 6;
pub const JMP_ABS_CYCLES: u32 = 3;
pub const JMP_IND_CYCLES: u32 = 5;

pub const RAM_SIZE: usize = 0x10000;
pub const VRAM_SIZE: usize = 0x4000;
pub const INTERNAL_RAM_SIZE: usize = 0x800;

pub const NAMETABLE_COUNT: usize = 0x10;
pub const NAMETABLE_SIZE: usize = 0x400;
pub const NMI_VECTOR: u16 = 0xFFFA;
pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;
pub const CLOCK_RATE_NTSC: u32 = 1789773;
pub const CLOCK_RATE_PAL: u32 = 1662607;
pub const CLOCK_RATE_DENDY: u32 = 1773448;
pub const PPU_DIVIDER: u64 = 4;
pub const CPU_DIVIDER: u64 = 12;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const PIXEL_COUNT: usize = 256 * 240;
pub const OUTPUT_BUFFER_SIZE: usize = 256 * 240 * 2;
pub const OAM_DECAY_CYCLE_COUNT: usize = 3000;

pub const PALETTE_RAM_BOOT_VALUES: &[u8] = &[
    0x09, 0x01, 0x00, 0x01, 0x00, 0x02, 0x02, 0x0D, 0x08, 0x10, 0x08, 0x24, 0x00, 0x00, 0x04, 0x2C,
    0x09, 0x01, 0x34, 0x03, 0x00, 0x04, 0x00, 0x14, 0x08, 0x3A, 0x00, 0x02, 0x00, 0x20, 0x2C, 0x08,
];

// pub const DONKEY_KONG: &[u8] = include_bytes!("../../data/nes/Donkey Kong (World) (Rev A).nes");
// pub const SUPER_MARIO: &[u8] = include_bytes!("../../data/nes/Super Mario Bros (E).nes");
pub const NES_TEST: &[u8] = include_bytes!("../test/nestest.nes");
