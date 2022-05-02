mod base_renderer;
mod webgl_renderer;

pub use base_renderer::*;
pub use webgl_renderer::*;
pub trait RenderingDevice {
    fn update_frame(&self, framebuffer: &mut [u8], width: usize, height: usize);
    fn render(&self);
    fn reset(&mut self);
    fn set_fullscreen_mode(
        &mut self,
        fullscreen: bool,
        monitor_width: usize,
        monitor_height: usize,
    );
}
