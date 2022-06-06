use crate::*;
use std::any::{Any, TypeId};

#[derive(Default)]
pub struct Console {
    auto_save_manager: AutoSaveManager,
    paused: bool,
    stop: bool,
    running: bool,
    stop_code: i32,
    pause_on_next_frame_requested: bool,
    reset_run_timers: bool,
    disable_oc_next_frame: bool,
    initialized: bool,
    pub battery_manager: BatteryManager,
    pub save_state_manager: SaveStateManager,
    pub video_decoder: VideoDecoder,
    pub video_renderer: VideoRenderer,
    pub notification_manager: NotificationManager,
    pub sound_mixer: SoundMixer,
    pub emulation_settings: EmulationSettings,
    pub debug_hud: DebugHud,
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub apu: Apu,
    pub mapper: Box<dyn Mapper>,
    pub control_manager: ControlManager,
    pub memory_manager: MemoryManager,
    pub cheat_manager: CheatManager,
    pub rewind_manager: RewindManager,
    pub history_viewer: HistoryViewer,
    pub system_action_manager: SystemActionManager,
    pub debugger: Debugger,
    pub rom_file: VirtualFile,
    pub patch_file: VirtualFile,
    pub nes_model: NesModel,
    pub internal_ram: InternalRam,
    pub open_bus: OpenBus,
}

impl Console {
    fn run_frame_with_run_ahead(&mut self, run_ahead_state: impl std::io::Write) {
        todo!()
    }
    fn load_hd_pack(&mut self, rom_file: VirtualFile, patch: VirtualFile) {
        todo!()
    }

    fn get_frame_delay(&self) -> f64 {
        todo!()
    }
    fn display_debug_information(
        &self,
        last_frame: f64,
        last_frame_min: &mut f64,
        last_frame_max: &mut f64,
        frame_durations: [f64; 60],
    ) {
        todo!()
    }

    fn export_stub(&self) {
        todo!()
    }

    pub fn process_cpu_clock(&mut self) {
        match self.mapper.id() {
            _ => BaseMapper::process_cpu_clock(self),
        }
        self.apu.process_cpu_clock();
    }

    pub fn new(rom_file: &VirtualFile, renderer: Box<dyn RenderingDevice>) -> Self {
        let mut this = Console::default();
        this.mapper = <dyn Mapper>::new(rom_file);
        this.cpu = Cpu::new();
        this.ppu = Ppu::new(EmulationFlags::default());
        this.emulation_settings = EmulationSettings::new();
        this.video_renderer = VideoRenderer::new(renderer);
        this.video_decoder = VideoDecoder::new(EmulationFlags::default());
        this.rewind_manager = RewindManager::new();
        Ppu::reset(&mut this);
        Cpu::reset(&mut this, false);
        this
    }

    pub fn save_batteries(&mut self) {
        todo!()
    }

    pub fn run(&mut self) {
        todo!()
    }

    pub fn reset_run_timers(&mut self) {
        todo!()
    }

    pub fn stop(&mut self) {
        todo!()
    }

    pub fn get_stop_code(&self) -> i32 {
        todo!()
    }

    pub fn run_single_frame(&mut self) {
        let last_frame_number = self.ppu.frame_count;
        while self.ppu.frame_count == last_frame_number {
            Cpu::exec(self);
        }
    }

    pub fn run_frame(&mut self) {
        let frame_count = self.ppu.frame_count;
        while self.ppu.frame_count == frame_count {
            Cpu::exec(self);
        }
    }

    pub fn update_hd_pack_mode(&mut self) -> bool {
        todo!()
    }

    pub fn get_dip_switch_count(&self) -> usize {
        todo!()
    }

    pub fn get_available_features(&self) -> ConsoleFeatures {
        todo!()
    }

    pub fn input_barcode(&mut self, barcode: u64, digit_count: u64) {
        todo!()
    }

    pub fn reset(&mut self, soft_reset: bool) {
        todo!()
    }

    pub fn power_cycle(&mut self) {
        todo!()
    }

    pub fn reload_rom(&mut self, for_power_cycle: bool) {
        todo!()
    }

    pub fn reset_components(&mut self, soft_reset: bool) {
        todo!()
    }

    pub fn pause(&mut self) {
        todo!()
    }

    pub fn resume(&mut self) {
        todo!()
    }

    pub fn stop_debuger(&mut self) {
        todo!()
    }

    pub fn save_state(save_stream: impl std::io::Write) {
        todo!()
    }

    pub fn load_state(load_stream: impl std::io::Read) {
        todo!()
    }

    pub fn get_lag_counter(&self) -> u32 {
        todo!()
    }

    pub fn reset_lag_counter(&mut self) {
        todo!()
    }

    pub fn is_running(&self) -> bool {
        todo!()
    }

    pub fn is_execution_stopped(&mut self) -> bool {
        todo!()
    }

    pub fn is_paused(&self) -> bool {
        todo!()
    }

    pub fn pause_onf_next_frame(&mut self) {
        todo!()
    }

    pub fn set_next_frame_overclock_status(&mut self, disabled: bool) {
        todo!()
    }

    pub fn is_debugger_attached(&self) -> bool {
        todo!()
    }

    pub fn get_fps(&self) -> f32 {
        todo!()
    }

    pub fn is_hd_ppu(&self) -> bool {
        todo!()
    }
}
