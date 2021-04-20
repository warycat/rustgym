use seed::{prelude::*, *};

use web_sys::{Location, Window};

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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
