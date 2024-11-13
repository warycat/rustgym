use crate::RenderingDevice;

#[derive(Default)]
pub struct WebGlRenderer {}

impl WebGlRenderer {
    fn new() -> Self {
        WebGlRenderer {}
    }
}

impl RenderingDevice for WebGlRenderer {
    fn update_frame(&self, _framebuffer: &mut [u8], _width: usize, _height: usize) {
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
        _fullscreen: bool,
        _monitor_width: usize,
        _monitor_height: usize,
    ) {
        todo!();
    }
}
