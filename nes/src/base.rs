pub type cstring = *const u8;
pub type wcstring = *const char;
pub type ibool = u32;
pub type Data = u32;
pub type Address = u32;
pub type Cycle = u32;
pub type Result = std::result::Result<i32, i32>;

/**
 * NTSC/PAL region mismatch.
 */
const RESULT_ERR_WRONG_MODE: Result = Result::Err(-13);
/**
 * Missing FDS BIOS.
 */
const RESULT_ERR_MISSING_BIOS: Result = Result::Err(-12);
/**
 * Unsupported or malformed mapper.
 */
const RESULT_ERR_UNSUPPORTED_MAPPER: Result = Result::Err(-11);
/**
 * Vs DualSystem is unsupported.
 */
const RESULT_ERR_UNSUPPORTED_VSSYSTEM: Result = Result::Err(-10);
/**
 * File format version is no longer supported.
 */
const RESULT_ERR_UNSUPPORTED_FILE_VERSION: Result = Result::Err(-9);
/**
 * Unsupported operation.
 */
const RESULT_ERR_UNSUPPORTED: Result = Result::Err(-8);
/**
 * Invalid CRC checksum.
 */
const RESULT_ERR_INVALID_CRC: Result = Result::Err(-7);
/**
 * Corrupt file.
 */
const RESULT_ERR_CORRUPT_FILE: Result = Result::Err(-6);
/**
 * Invalid file.
 */
const RESULT_ERR_INVALID_FILE: Result = Result::Err(-5);
/**
 * Invalid parameter(s).
 */
const RESULT_ERR_INVALID_PARAM: Result = Result::Err(-4);
/**
 * System not ready.
 */
const RESULT_ERR_NOT_READY: Result = Result::Err(-3);
/**
 * Out of memory.
 */
const RESULT_ERR_OUT_OF_MEMORY: Result = Result::Err(-2);
/**
 * Generic error.
 */
const RESULT_ERR_GENERIC: Result = Result::Err(-1);
/**
 * Success.
 */
const RESULT_OK: Result = Result::Ok(0);
/**
 * Success but operation had no effect.
 */
const RESULT_NOP: Result = Result::Ok(1);
/**
 * Success but image dump may be bad.
 */
const RESULT_WARN_BAD_DUMP: Result = Result::Ok(2);
/**
 * Success but PRG-ROM may be bad.
 */
const RESULT_WARN_BAD_PROM: Result = Result::Ok(3);
/**
 * Success but CHR-ROM may be bad.
 */
const RESULT_WARN_BAD_CROM: Result = Result::Ok(4);
/**
 * Success but file header may have incorrect data.
 */
const RESULT_WARN_BAD_FILE_HEADER: Result = Result::Ok(5);
/**
 * Success but save data has been lost.
 */
const RESULT_WARN_SAVEDATA_LOST: Result = Result::Ok(6);
/**
 * Success but data may have been replaced.
 */
const RESULT_WARN_DATA_REPLACED: Result = Result::Ok(8);

pub enum Region {
    REGION_NTSC,
    REGION_PAL,
}

pub enum System {
    SYSTEM_NES_NTSC,
    SYSTEM_NES_PAL,
    SYSTEM_NES_PAL_A,
    SYSTEM_NES_PAL_B,
    SYSTEM_FAMICOM,
    SYSTEM_DENDY,
    SYSTEM_VS_UNISYSTEM,
    SYSTEM_VS_DUALSYSTEM,
    SYSTEM_PLAYCHOICE_10,
}

pub enum FavoredSystem {
    FAVORED_NES_NTSC,
    FAVORED_NES_PAL,
    FAVORED_FAMICOM,
    FAVORED_DENDY,
}

pub enum CpuModel {
    CPU_RP2A03,
    CPU_RP2A07,
    CPU_DENDY,
}

pub enum PpuModel {
    PPU_RP2C02,
    PPU_RP2C03B,
    PPU_RP2C03G,
    PPU_RP2C04_0001,
    PPU_RP2C04_0002,
    PPU_RP2C04_0003,
    PPU_RP2C04_0004,
    PPU_RC2C03B,
    PPU_RC2C03C,
    PPU_RC2C05_01,
    PPU_RC2C05_02,
    PPU_RC2C05_03,
    PPU_RC2C05_04,
    PPU_RC2C05_05,
    PPU_RP2C07,
    PPU_DENDY,
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

pub const SIZE_1K: usize = 0x400;
pub const SIZE_2K: usize = 0x800;
pub const SIZE_4K: usize = 0x1000;
pub const SIZE_5K: usize = 0x1400;
pub const SIZE_6K: usize = 0x1800;
pub const SIZE_8K: usize = 0x2000;
pub const SIZE_16K: usize = 0x4000;
pub const SIZE_32K: usize = 0x8000;
pub const SIZE_40K: usize = 0xA000;
pub const SIZE_64K: usize = 0x10000;
pub const SIZE_128K: usize = 0x20000;
pub const SIZE_256K: usize = 0x40000;
pub const SIZE_512K: usize = 0x80000;
pub const SIZE_1024K: usize = 0x100000;
pub const SIZE_2048K: usize = 0x200000;
pub const SIZE_3072K: usize = 0x300000;
pub const SIZE_4096K: usize = 0x400000;
pub const SIZE_8192K: usize = 0x800000;
pub const SIZE_16384K: usize = 0x1000000;

pub const NMI_VECTOR: u16 = 0xFFFA;
pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;
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
