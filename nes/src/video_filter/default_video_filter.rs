use crate::*;
pub struct DefaultVideoFilter {
    base_video_filter: BaseVideoFilter,
}

impl VideoFilter for DefaultVideoFilter {
    fn base_video_filter(&self) -> &BaseVideoFilter {
        &self.base_video_filter
    }

    fn base_video_filter_mut(&mut self) -> &mut BaseVideoFilter {
        &mut self.base_video_filter
    }
}
