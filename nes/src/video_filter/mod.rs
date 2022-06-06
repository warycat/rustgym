use crate::*;

mod default_video_filter;

pub use default_video_filter::*;

pub trait VideoFilter {
    fn base_video_filter_mut(&mut self) -> &mut BaseVideoFilter;
    fn base_video_filter(&self) -> &BaseVideoFilter;

    fn apply_filter(&self, ppu_output_buffer: &mut Vec<u16>) {
        todo!()
    }

    fn on_before_apply_filter(&self) {
        todo!()
    }

    fn is_odd_frame(&self) -> bool {
        todo!()
    }

    fn send_frame(&self, ppu_output_buffer: &mut Vec<u16>, frame_number: u32) {
        todo!()
    }

    fn take_screenshot(&self, rom_name: String, filter_type: VideoFilterType) {
        todo!()
    }

    fn get_overscan_dimensions(&self) -> OverscanDimensions {
        todo!()
    }

    fn get_frame_info(&self) -> FrameInfo {
        todo!()
    }
}

pub struct BaseVideoFilter {
    pub buffer_size: usize,
    pub overscan: OverscanDimensions,
    pub is_odd_frame: bool,
}
