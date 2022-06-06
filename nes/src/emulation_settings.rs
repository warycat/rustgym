use crate::*;
use std::collections::HashMap;

const SPEED_VALUES: &[u32] = &[
    1, 3, 6, 12, 25, 50, 75, 100, 150, 200, 250, 300, 350, 400, 450, 500, 750, 1000, 2000, 4000,
];

const VERSION: (u16, u8, u8) = (0, 9, 9);

bitflags! {
    #[derive(Default)]
    pub struct EmulationFlags: u64 {
        const Paused = 0x01;
        const ShowFPS = 0x02;
        const VerticalSync = 0x04;
        const AllowInvalidInput = 0x08;
        const RemoveSpriteLimit = 0x10;
        const UseHdPacks = 0x20;
        const HasFourScore = 0x40;
        const DisableDynamicSampleRate = 0x80;
        const PauseOnMovieEnd = 0x0100;
        const EnablePpuOamRowCorruption = 0x0200;
        const AllowBackgroundInput = 0x0400;
        const ReduceSoundInBackground = 0x0800;
        const MuteSoundInBackground = 0x1000;
        const FdsFastForwardOnLoad = 0x2000;
        const FdsAutoLoadDisk = 0x4000;
        const Mmc3IrqAltBehavior = 0x8000;
        const SwapDutyCycles = 0x10000;
        const DisableGameSelectionScreen = 0x20000;
        const AutoConfigureInput = 0x40000;
        const ShowLagCounter = 0x80000;
        const SilenceTriangleHighFreq = 0x100000;
        const ReduceDmcPopping = 0x200000;
        const DisableBackground = 0x400000;
        const DisableSprites = 0x800000;
        const ForceBackgroundFirstColumn = 0x1000000;
        const ForceSpritesFirstColumn = 0x2000000;
        const DisablePpu2004Reads = 0x4000000;
        const DisableNoiseModeFlag = 0x8000000;
        const DisablePaletteRead = 0x10000000;
        const DisableOamAddrBug = 0x20000000;
        const DisablePpuReset = 0x40000000;
        const EnableOamDecay = 0x80000000;
        const UseNes101Hvc101Behavior = 0x100000000;
        const ShowFrameCounter = 0x200000000;
        const ShowGameTimer = 0x400000000;
        const FdsAutoInsertDisk = 0x800000000;
        const Rewind =  0x1000000000;
        const Turbo = 0x2000000000;
        const InBackground = 0x4000000000;
        const NsfPlayerEnabled = 0x8000000000;
        const DisplayMovieIcons = 0x10000000000;
        const HidePauseOverlay = 0x20000000000;
        const UseCustomVsPalette = 0x40000000000;
        const AdaptiveSpriteLimit = 0x80000000000;
        const EnablePpu2006ScrollGlitch = 0x100000000000;
        const EnablePpu2000ScrollGlitch = 0x200000000000;
        const ConfirmExitResetPower = 0x400000000000;
        const NsfRepeat = 0x800000000000;
        const NsfShuffle = 0x1000000000000;
        const IntegerFpsMode = 0x2000000000000;
        const DebuggerWindowEnabled = 0x4000000000000;
        const BreakOnCrash = 0x8000000000000;
        const AllowMismatchingSaveState = 0x10000000000000;
        const RandomizeMapperPowerOnState = 0x20000000000000;
        const UseHighResolutionTimer = 0x40000000000000;
        const DisplayDebugInfo = 0x80000000000000;
        const ReduceSoundInFastForward = 0x100000000000000;
        const VsDualMuteMaster = 0x200000000000000;
        const VsDualMuteSlave = 0x400000000000000;
        const RandomizeCpuPpuAlignment = 0x800000000000000;
        const ForceMaxSpeed = 0x4000000000000000;
        const ConsoleMode = 0x8000000000000000;
    }
}

pub enum Language {
    English,
    French,
    Japanese,
    Russian,
    Spanish,
    Ukrainian,
    Portuguese,
    Catalan,
    Chinese,
    Italian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

pub enum EqualizerFilter {
    None,
    Butterworth,
    Chebyshev1,
    Chebyshev2,
}

impl Default for EqualizerFilter {
    fn default() -> Self {
        EqualizerFilter::None
    }
}

pub enum StereoFilter {
    None,
    Delay,
    Panning,
    CombFilter,
}

impl Default for StereoFilter {
    fn default() -> Self {
        StereoFilter::None
    }
}

#[derive(Default)]
pub struct AudioFilterSettings {
    filter: StereoFilter,
    angle: f64,
    delay: i32,
    strength: i32,
    reverb_delay: f64,
    reverb_strength: f64,
    cross_fade_ratio: i32,
}

#[derive(Default)]
pub struct OverscanDimensions {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

impl OverscanDimensions {
    fn get_pixel_count(&self) -> u32 {
        self.get_screen_width() * self.get_screen_height()
    }
    fn get_screen_width(&self) -> u32 {
        256 - self.left - self.right
    }
    fn get_screen_height(&self) -> u32 {
        240 - self.top - self.bottom
    }
}

#[derive(Clone, Copy)]
pub enum VideoFilterType {
    None,
    Ntsc,
    BisqwitNtscQuarterRes,
    BisqwitNtscHalfRes,
    BisqwitNtsc,
    Xbrz2x,
    Xbrz3x,
    Xbrz4x,
    Xbrz5x,
    Xbrz6x,
    Hq2x,
    Hq3x,
    Hq4x,
    Scale2x,
    Scale3x,
    Scale4x,
    Sai2x,
    SuperSai2x,
    SuperEagle,
    Prescale2x,
    Prescale3x,
    Prescale4x,
    Prescale6x,
    Prescale8x,
    Prescale10x,
    Raw,
    HdPack,
}

impl Default for VideoFilterType {
    fn default() -> Self {
        VideoFilterType::None
    }
}

pub enum VideoAspectRatio {
    NoStretching,
    Auto,
    Ntsc,
    Pal,
    Standard,
    Widescreen,
    Custom,
}

impl Default for VideoAspectRatio {
    fn default() -> Self {
        VideoAspectRatio::NoStretching
    }
}

pub enum VideoResizeFilter {
    NearestNeighbor,
    Bilinear,
}

impl Default for VideoResizeFilter {
    fn default() -> Self {
        VideoResizeFilter::NearestNeighbor
    }
}

#[derive(Default)]
pub struct PictureSettings {
    brightness: f64,
    contrast: f64,
    saturation: f64,
    hue: f64,
    scanline_intensity: f64,
}

#[derive(Default)]
pub struct NtscFilterSettings {
    sharpness: f64,
    gamma: f64,
    resolution: f64,
    artifacts: f64,
    fringing: f64,
    bleed: f64,
    merge_fields: bool,
    vertical_blend: bool,
    keep_vertical_resolution: bool,
    y_filter_length: f64,
    i_filter_length: f64,
    q_filter_length: f64,
}

pub enum ConsoleType {
    Nes,
    Famicom,
}

impl Default for ConsoleType {
    fn default() -> Self {
        ConsoleType::Nes
    }
}

pub enum ExpansionPortDevice {
    None,
    Zapper,
    FourPlayerAdapter,
    ArkanoidController,
    OekaKidsTablet,
    FamilyTrainerMat,
    KonamiHyperShot,
    FamilyBasicKeyboard,
    PartyTap,
    Pachinko,
    ExcitingBoxing,
    JissenMahjong,
    SuborKeyboard,
    BarcodeBattler,
    HoriTrack,
    BandaiHyperShot,
    AsciiTurboFile,
    BattleBox,
}

impl Default for ExpansionPortDevice {
    fn default() -> Self {
        ExpansionPortDevice::None
    }
}

pub enum InputDisplayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for InputDisplayPosition {
    fn default() -> Self {
        InputDisplayPosition::TopLeft
    }
}

#[derive(Default)]
pub struct InputDisplaySettings {
    visible_ports: u8,
    display_position: InputDisplayPosition,
    display_horizontally: bool,
}

pub enum RamPowerOnState {
    AllZeros,
    AllOnes,
    Random,
}

impl Default for RamPowerOnState {
    fn default() -> Self {
        RamPowerOnState::AllZeros
    }
}

#[derive(Default)]
pub struct EmulationSettings {
    is_full_color_palette: bool,
    audio_settings_changed: bool,
    volume_reduction: f64,
    custom_aspect_ratio: f64,
    need_controller_update: bool,
    controller_deadzone_size: u32,
    pub ppu_palette_argb: Palette,
    flags: EmulationFlags,
    pub display_language: Language,
    pub nes_model: NesModel,
    pub ppu_model: PpuModel,
    pub channel_volumes: Vec<f64>,
    pub master_volume: f64,
    pub channel_pannings: Vec<f64>,
    pub band_gains: Vec<f64>,
    pub bands: Vec<f64>,
    pub equalizer_bands: Vec<f64>,
    pub equalizer_filter: EqualizerFilter,
    pub sample_rate: u32,
    pub audio_latency: u32,
    pub audio_filter_settings: AudioFilterSettings,
    pub run_ahead_frames: u32,
    pub is_run_ahead_frame: bool,
    pub emulation_speed: u32,
    pub turbo_speed: u32,
    pub rewind_speed: u32,
    pub rewind_buffer_size: u32,
    pub disable_overclocking: bool,
    pub extra_scanlines_before_nmi: u32,
    pub extra_scanlines_after_nmi: u32,
    pub overscan: OverscanDimensions,
    pub video_filter_type: VideoFilterType,
    pub resize_filter: VideoResizeFilter,
    pub video_aspect_ratio: VideoAspectRatio,
    pub background_enabled: bool,
    pub sprites_enabled: bool,
    pub picture_settings: PictureSettings,
    pub ntsc_filter_settings: NtscFilterSettings,
    pub video_scale: f64,
    pub screen_rotation: u32,
    pub exclusive_refresh_rate: u32,
    pub expansion_port_device: ExpansionPortDevice,
    pub console_type: ConsoleType,
    pub controller_types: Vec<ControllerType>,
    pub mouse_sensitivity: HashMap<i32, f64>,
    pub zapper_detection_radius: u32,
    pub input_poll_scanline: i32,
    pub nsf_auto_detect_silence_delay: i32,
    pub nsf_move_to_next_track_time: i32,
    pub nsf_disable_apt_irq: bool,
    pub input_display_settings: InputDisplaySettings,
    pub ram_power_on_state: RamPowerOnState,
    pub auto_save_delay: u32,
    pub dip_switches: u32,
    pub keyboard_mode_enabled: bool,
    pub pause_screen_message: String,
}

impl EmulationSettings {
    pub fn new() -> Self {
        let mut this = EmulationSettings::default();
        this.audio_latency = 50;
        this.channel_volumes = vec![1.0; 11];
        this.channel_pannings = vec![1.0; 11];
        this.band_gains = vec![0.0; 20];
        this.bands = vec![
            40.0, 56.0, 80.0, 113.0, 160.0, 225.0, 320.0, 450.0, 600.0, 750.0, 1000.0, 2000.0,
            3000.0, 4000.0, 5000.0, 6000.0, 7000.0, 10000.0, 12500.0, 15000.0,
        ];
        this.master_volume = 1.0;
        this.volume_reduction = 0.75;
        this.sample_rate = 48000;
        this.emulation_speed = 100;
        this.turbo_speed = 300;
        this.rewind_speed = 100;
        this.rewind_buffer_size = 300;
        this.video_scale = 1.0;
        this.custom_aspect_ratio = 1.0;
        this.background_enabled = true;
        this.sprites_enabled = true;
        this.exclusive_refresh_rate = 60;
        this.controller_deadzone_size = 2;
        this.input_poll_scanline = 241;
        this.nsf_auto_detect_silence_delay = 3000;
        this.nsf_move_to_next_track_time = 120;
        this.nsf_disable_apt_irq = true;
        this.auto_save_delay = 5;
        this
    }

    fn get_version() -> u32 {
        (VERSION.0 as u32) << 16 | (VERSION.1 as u32) << 8 | VERSION.0 as u32
    }

    fn get_version_string() -> String {
        format!("v{}.{}.{}", VERSION.0, VERSION.1, VERSION.2)
    }
}
