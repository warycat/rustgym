mod utils;
use rustgym_msg::Msg;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, Location, MessageEvent, WebSocket, Window};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn wsurl() -> Result<String, JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
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

#[wasm_bindgen(start)]
pub fn start_websocket() -> Result<(), JsValue> {
    utils::set_panic_hook();
    let wsurl = wsurl()?;
    console_log!("{}", wsurl);
    let ws = WebSocket::new(&wsurl)?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let json: String = txt.into();
            let msg: Msg = serde_json::from_str(&json).expect("json");
            console_log!("message event, received Text: {:?}", msg);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        let msg = serde_json::to_string(&Msg::ReqRegistorClient).expect("registor");
        match cloned_ws.send_with_str(&msg) {
            Ok(x) => console_log!("message successfully sent {:?}", x),
            Err(err) => console_log!("error sending message: {:?}", err),
        };
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(add(1, 2), 3);
    }
}
