use crate::*;
use bitflags::bitflags;
use std::any::Any;
use std::convert::From;

const STANDARD_MAPPING: [usize; 8] = [0, 1, 8, 9, 12, 13, 14, 15];

pub struct StandardController {
    buttons: u8,
    base_device: BaseDevice,
}

impl StandardController {
    pub fn new() -> Self {
        let buttons = 0;
        let base_device = BaseDevice::default();
        StandardController {
            base_device,
            buttons,
        }
    }

    pub fn set_buttons(&mut self, gamepad: Gamepad) {
        for i in 0..8 {
            let key = STANDARD_MAPPING[i];
            if gamepad.pressed[key] {
                self.buttons |= 1 << i;
            }
        }
    }
}

impl Device for StandardController {
    fn base_device(&self) -> &BaseDevice {
        &self.base_device
    }
    fn base_device_mut(&mut self) -> &mut BaseDevice {
        &mut self.base_device
    }

    fn set_state_from_input(&mut self) {
        todo!()
    }

    fn refresh_state_buffer(&mut self) {
        todo!()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl MemoryHandler for StandardController {
    fn read_byte(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn write_byte(&mut self, addr: u16, byte: u8) {
        todo!()
    }
}
