use crate::ControlManager;

#[derive(Default)]
pub struct RewindManager {}

impl RewindManager {
    pub fn new() -> Self {
        let this = RewindManager::default();
        this
    }

    pub fn send_frame(&mut self, ppu_output_buffer: &mut Vec<u16>, synchronous: bool) {
        todo!()
    }
}
