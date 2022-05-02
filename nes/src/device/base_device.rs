use crate::*;

#[derive(Default, Clone)]
pub struct ControlDeviceState(Vec<u8>);

#[derive(Default)]
pub struct BaseDevice {
    state: ControlDeviceState,
    pub strobe: bool,
}

impl BaseDevice {
    pub fn get_raw_state(&self) -> ControlDeviceState {
        self.state.clone()
    }

    pub fn clear_state(&mut self) {
        self.state = ControlDeviceState::default();
    }
}
