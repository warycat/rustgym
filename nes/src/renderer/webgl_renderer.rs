use crate::RenderingDevice;

#[derive(Default)]
pub struct WebGlRenderer {}

impl WebGlRenderer {
    fn new() -> Self {
        WebGlRenderer {}
    }
}

impl RenderingDevice for WebGlRenderer {
    fn update_frame(&self, framebuffer: &mut [u8], width: usize, height: usize) {
        todo!();
    }
    fn render(&self) {
        todo!();
    }
    fn reset(&mut self) {
        todo!();
    }
    fn set_fullscreen_mode(
        &mut self,
        fullscreen: bool,
        monitor_width: usize,
        monitor_height: usize,
    ) {
        todo!();
    }
}
