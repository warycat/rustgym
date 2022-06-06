use crate::*;

#[derive(Default)]
pub struct ScreenSize {
    width: usize,
    height: usize,
    scale: f64,
}

#[derive(Default)]
pub struct VideoDecoder {
    settings: EmulationFlags,
    frame_number: u32,
    hud: VideoHud,
    pub frame_count: u32,
    previous_screen_size: ScreenSize,
    previous_scale: f64,
    pub frame_info: FrameInfo,
    video_filter_type: VideoFilterType,
    video_filter: Option<Box<dyn VideoFilter>>,
    scale_filter: ScaleFilter,
    rotate_filter: RotateFilter,
}

impl VideoDecoder {
    fn decode_thread(&mut self) {
        todo!()
    }

    pub fn new(settings: EmulationFlags) -> Self {
        let mut this = VideoDecoder::default();
        this.settings = settings;
        this
    }

    fn decode_frame(console: &mut Console, synchronous: bool) {
        let video_filter_type = console.emulation_settings.video_filter_type;
        console.video_decoder.video_filter = match video_filter_type {
            VideoFilterType::None => None,
            _ => None,
        };

        if let Some(video_filter) = console.video_decoder.video_filter.as_ref() {
            video_filter.send_frame(
                &mut console.ppu.current_output_buffer(),
                console.video_decoder.frame_number,
            );
        }
        let screen_size = console.video_decoder.get_screen_size(true);

        console
            .rewind_manager
            .send_frame(console.ppu.current_output_buffer(), synchronous);
    }

    fn take_screenshot(&mut self) {
        todo!()
    }

    fn get_screen_size(&self, ignore_scale: bool) -> ScreenSize {
        todo!()
    }

    pub fn update_frame_sync(console: &mut Console) {
        if console.emulation_settings.is_run_ahead_frame {
            return;
        }
        console.video_decoder.frame_number = console.ppu.frame_count;
        VideoDecoder::decode_frame(console, true);
        console.video_decoder.frame_count += 1;
    }

    fn update_frame(&self, ppu_output_buffer: &mut Vec<u16>) {
        todo!()
    }

    fn is_running(&self) -> bool {
        todo!()
    }

    fn start_thread(&mut self) {
        todo!()
    }

    fn stop_thread(&mut self) {
        todo!()
    }
}
