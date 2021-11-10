use crate::base::*;
use crate::hook::*;
use crate::sound::Sound;
use crate::state::SaveLoad;

pub const CYCLE_MAX: u32 = !0;
pub const RAM_SIZE: usize = SIZE_2K;

pub enum IrqLine {
    IRQ_EXT = 0x01,
    IRQ_FRAME = 0x40,
    IRQ_DMC = 0x80,
}

pub enum Level {
    LEVEL_LOW = 1,
    LEVEL_HIGH = 9,
    LEVEL_HIGHEST = 10,
}

pub trait Cpu: SaveLoad {
    fn reset(&mut self);
    fn set_ram_power_state(&mut self, power_state: u32);
    fn boot(&mut self, hard: bool);
    fn execute_frame(&mut self, sound: &impl Sound);
    fn end_frame(&mut self);
    fn power_off(&mut self);
    fn do_mni(&mut self, cycle: Cycle);
    fn do_irq(&mut self, irq_line: IrqLine, cycle: Cycle);
    fn peek(&self, address: usize) -> u32;
    fn poke(&self, address: usize, data: u32);
    fn is_odd_cycle(&self) -> bool;
    fn is_write_cycle(&self, cycle: Cycle) -> bool;
    fn get_clock_base(&self) -> Cycle;
    fn get_clock_divider(&self) -> u32;
    fn get_time(cycle: Cycle) -> u32;
    fn get_fps(&self) -> u32;
    fn set_model(&mut self, model: CpuModel);
    fn add_hook(hook: &impl Hook);
    fn remove_hook(hook: &impl Hook);
}
