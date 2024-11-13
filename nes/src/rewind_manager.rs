#[derive(Default)]
pub struct RewindManager {}

impl RewindManager {
    pub fn new() -> Self {
        let this = RewindManager::default();
        this
    }

    pub fn send_frame(&mut self, _ppu_output_buffer: &mut Vec<u16>, _synchronous: bool) {
        todo!()
    }
}
