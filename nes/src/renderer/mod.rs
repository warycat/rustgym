use crate::*;

mod webgl_renderer;

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

#[derive(Default)]
pub struct BaseRenderer {
    _fps_timer: Timer,
    _last_frame_count: u32,
    _last_rendered_frame_count: u32,
    _current_fps: u32,
    _current_rendered_fps: u32,
}

impl RenderingDevice for BaseRenderer {
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

impl BaseRenderer {
    fn remove_old_toasts(&self) {
        todo!();
    }

    fn wrap_text(_utf8_text: String, _max_line_width: f64, _line_count: u32) -> String {
        todo!()
    }

    fn measure_string(&self, _text: String) -> f64 {
        todo!()
    }

    fn contains_character(&self, _character: char) -> bool {
        todo!()
    }

    fn is_message_shown(&self) -> bool {
        todo!()
    }

    fn display_message(&self, _title: String, _message: String) {
        todo!()
    }

    fn draw_toast(&self) {
        todo!();
    }

    fn draw_string(
        &self,
        _message: String,
        _x: i32,
        _y: i32,
        _r: u8,
        _g: u8,
        _b: u8,
        _opacity: u8,
    ) {
        todo!();
    }

    fn show_fps_counter(_console: &mut Console, _line_number: i32) {
        todo!();
    }

    fn show_lag_counter(_console: &mut Console, _line_number: i32) {
        todo!();
    }

    fn show_frame_counter(_console: &mut Console, _line_number: i32) {
        todo!();
    }

    fn show_game_counter(_console: &mut Console, _line_number: i32) {
        todo!();
    }

    fn show_counters(_console: &mut Console) {
        todo!();
    }
}
