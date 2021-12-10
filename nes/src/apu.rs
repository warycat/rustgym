use crate::base::*;
use crate::sound::Sound;
use crate::state::SaveLoad;

pub trait Apu: SaveLoad {
    fn reset(&mut self, hard: bool);
    fn power_off(&mut self);
    fn clean_buffers(&mut self);
    fn begin_frame(&mut self, sound: &impl Sound);
    fn end_frame(&mut self);
    fn write_frame_ctrl(&mut self, data: u32);
    fn clock(&self) -> Cycle;
    fn clock_dma(&self, read_address: u32);
    fn set_sample_rate(&mut self, rate: u32) -> Result<NesWarning, NesError>;
    fn set_sample_bits(&mut self, bits: u32) -> Result<NesWarning, NesError>;
    fn set_speed(&mut self, speed: u32) -> Result<NesWarning, NesError>;
    fn set_volume(&mut self, channel: u32, volume: u32) -> Result<NesWarning, NesError>;
    fn get_volume(&self, channel: u32) -> u32;
    fn get_ctrl(&self) -> u32;
    fn mute(&mut self, mute: bool);
    fn set_auto_transpose(&mut self, transpose: bool);
    fn set_genie(&mut self, genie: bool);
    fn enable_stereo(&mut self, enable: bool);
}

pub trait Channel {
    fn update(&self);
    fn connect(&self, audible: bool);
    fn get_volume(&self, channel: u32) -> u32;
    fn get_oscillator_clock(rate: &Cycle, fixed: &u32);
    fn get_cpu_clock_base(&self) -> Cycle;
    fn get_cpu_clock_divider(&self) -> u32;
    fn get_cpu_clock(&self, clock: u32) -> Cycle;
    fn is_muted(&self) -> bool;
    fn is_genie(&self) -> bool;
}
