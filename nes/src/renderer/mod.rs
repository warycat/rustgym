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
    fps_timer: Timer,
    last_frame_count: u32,
    last_rendered_frame_count: u32,
    current_fps: u32,
    current_rendered_fps: u32,
}

impl RenderingDevice for BaseRenderer {
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

impl BaseRenderer {
    fn remove_old_toasts(&self) {
        todo!();
    }

    fn wrap_text(utf8_text: String, max_line_width: f64, line_count: u32) -> String {
        todo!()
    }

    fn measure_string(&self, text: String) -> f64 {
        todo!()
    }

    fn contains_character(&self, character: char) -> bool {
        todo!()
    }

    fn is_message_shown(&self) -> bool {
        todo!()
    }

    fn display_message(&self, title: String, message: String) {
        todo!()
    }

    fn draw_toast(&self) {
        todo!();
    }

    fn draw_string(&self, message: String, x: i32, y: i32, r: u8, g: u8, b: u8, opacity: u8) {
        todo!();
    }

    fn show_fps_counter(console: &mut Console, line_number: i32) {
        todo!();
    }

    fn show_lag_counter(console: &mut Console, line_number: i32) {
        todo!();
    }

    fn show_frame_counter(console: &mut Console, line_number: i32) {
        todo!();
    }

    fn show_game_counter(console: &mut Console, line_number: i32) {
        todo!();
    }

    fn show_counters(console: &mut Console) {
        todo!();
    }
}
