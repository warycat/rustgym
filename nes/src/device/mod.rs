use crate::*;
use std::any::Any;

mod standard_controler;

pub use standard_controler::*;

pub trait Device: MemoryHandler {
    fn base_device_mut(&mut self) -> &mut BaseDevice;
    fn base_device(&self) -> &BaseDevice;
    fn refresh_state_buffer(&mut self);
    fn set_state_from_input(&mut self);

    fn strobe_process_read(&mut self) {
        if self.base_device().strobe {
            self.refresh_state_buffer();
        }
    }

    fn strobe_process_write(&mut self, byte: u8) {
        let proto = self.base_device_mut();
        let prev_strobe = proto.strobe;
        proto.strobe = byte & 0x01 == 0x01;
        if prev_strobe && !proto.strobe {
            self.refresh_state_buffer();
        }
    }

    fn get_raw_state(&self) -> ControlDeviceState {
        self.base_device().get_raw_state()
    }

    fn clear_state(&mut self) {
        self.base_device_mut().clear_state();
    }

    fn as_any(&mut self) -> &mut dyn Any;
}

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
