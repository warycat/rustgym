use wasm_bindgen::JsCast;
use web_sys::GamepadButton;

pub struct Gamepad {
    pub id: String,
    pub index: u32,
    pub pressed: Vec<bool>,
    pub touched: Vec<bool>,
    pub value: Vec<f64>,
    pub axes: Vec<f64>,
    pub timestamp: f64,
}

impl From<web_sys::Gamepad> for Gamepad {
    fn from(gamepad: web_sys::Gamepad) -> Gamepad {
        let id = gamepad.id();
        let index = gamepad.index();
        let axes = gamepad.axes().to_vec();
        let axes = axes.into_iter().map(|v| v.as_f64().unwrap()).collect();
        let buttons = gamepad.buttons().to_vec();
        let buttons: Vec<GamepadButton> =
            buttons.into_iter().map(|b| b.dyn_into().unwrap()).collect();
        let pressed = buttons.iter().map(|b| b.pressed()).collect();
        let touched = buttons.iter().map(|b| b.touched()).collect();
        let value = buttons.iter().map(|b| b.value()).collect();
        let timestamp = gamepad.timestamp();
        Gamepad {
            id,
            index,
            pressed,
            touched,
            value,
            axes,
            timestamp,
        }
    }
}
