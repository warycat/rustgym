use seed::{prelude::*, *};

use web_sys::{Document, Location, Navigator, Window};

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window().document().unwrap()
}

pub fn navigator() -> Navigator {
    window().navigator()
}

pub fn wsurl() -> Result<String, JsValue> {
    let window: Window = window();
    let location: Location = window.location();
    let protocol: String = location.protocol()?;
    let host: String = location.host()?;
    let ws_protocol = if protocol == "https:" {
        "wss://"
    } else {
        "ws://"
    };
    Ok(format!("{}{}/ws/", ws_protocol, host))
}
