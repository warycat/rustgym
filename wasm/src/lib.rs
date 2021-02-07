mod utils;
use rustgym_message::Message;
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
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
            })
                as Box<dyn FnMut(web_sys::ProgressEvent)>);
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
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
        match cloned_ws.send_with_str("open") {
            Ok(x) => console_log!("message successfully sent {:?}", x),
            Err(err) => console_log!("error sending message: {:?}", err),
        };
        let msg = serde_json::to_string(&Message::Ping).expect("ping");
        match cloned_ws.send_with_str(&msg) {
            Ok(x) => console_log!("message successfully sent {:?}", x),
            Err(err) => console_log!("error sending message: {:?}", err),
        };
        let msg = serde_json::to_string(&Message::Login {
            name: "larry".to_string(),
        })
        .expect("login");
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
