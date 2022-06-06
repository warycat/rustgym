use crate::*;
use log::info;
use std::any::Any;
use std::collections::HashMap;

const HOME: usize = 16;
pub enum ControllerType {
    None,
    StandardController,
    Zapper,
    ArkanoidController,
    SnesController,
    PowerPad,
    SnesMouse,
    SuborMouse,
    VsZapper,
    VbController,
}

impl Default for ControllerType {
    fn default() -> Self {
        ControllerType::None
    }
}

#[derive(Default)]
pub struct ControlManager {
    settings: EmulationFlags,
    control_devices: HashMap<u32, StandardController>,
    input_poll: Option<Box<dyn Fn() -> Vec<Gamepad>>>,
}

impl ControlManager {
    pub fn new(settings: EmulationFlags, input_poll: impl Fn() -> Vec<Gamepad> + 'static) -> Self {
        let mut this = ControlManager::default();
        this.settings = settings;
        this.input_poll = Some(Box::new(input_poll));
        this
    }

    pub fn update_input_state(&mut self) {
        if let Some(cb) = self.input_poll.as_ref() {
            let gamepads = cb();
            for device in self.control_devices.values_mut() {
                device.clear_state();
            }
            for gamepad in gamepads {
                let index = gamepad.index;
                let standard_controller = self
                    .control_devices
                    .entry(index)
                    .or_insert(StandardController::new());

                standard_controller.set_buttons(gamepad);
            }
        }
    }

    fn get_port_states(&self) -> Vec<ControlDeviceState> {
        let mut res = vec![];
        for i in 0..4 {
            if let Some(device) = self.control_devices.get(&i) {
                res.push(device.get_raw_state());
            } else {
                res.push(ControlDeviceState::default());
            }
        }
        res
    }

    fn get_open_bus_mask(console: &Console, port: u8) -> u8 {
        match console.emulation_settings.console_type {
            ConsoleType::Nes => {
                if console
                    .control_manager
                    .settings
                    .contains(EmulationFlags::UseNes101Hvc101Behavior)
                {
                    if port == 0 {
                        0xE4
                    } else {
                        0xE0
                    }
                } else {
                    0xE0
                }
            }
            ConsoleType::Famicom => {
                if port == 0 {
                    0xF8
                } else {
                    0xE0
                }
            }
        }
    }

    pub fn read_byte(console: &mut Console, addr: u16) -> u8 {
        let port = (addr - 0x4016) as u8;
        let mut byte =
            console.open_bus.get_open_bus() & ControlManager::get_open_bus_mask(console, port);
        for device in console.control_manager.control_devices.values_mut() {
            byte |= device.read_byte(addr);
        }
        byte
    }

    pub fn write_byte(console: &mut Console, addr: u16, byte: u8) {
        for device in console.control_manager.control_devices.values_mut() {
            device.write_byte(addr, byte);
        }
    }
}
