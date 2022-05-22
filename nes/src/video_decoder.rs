use crate::*;

#[derive(Default)]
pub struct ScreenSize {
    width: usize,
    height: usize,
    scale: f64,
}

#[derive(Default)]
pub struct VideoDecoder {
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
    fn update_video_filter(&mut self) {
        todo!()
    }

    fn decode_thread(&mut self) {
        todo!()
    }

    pub fn new() -> Self {
        let this = VideoDecoder::default();
        this
    }

    fn decode_frame(console: &mut Console, synchronous: bool) {
        todo!()
        // console.rewind_manager.send_frame();
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
