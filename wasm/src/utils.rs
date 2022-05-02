use js_sys::Uint8Array;
use rustgym_msg::ClientInfo;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{
    window, Blob, ConstrainLongRange, Document, GamepadButton, HtmlAnchorElement,
    HtmlButtonElement, HtmlCanvasElement, HtmlDivElement, HtmlElement, HtmlInputElement,
    HtmlLiElement, HtmlParagraphElement, HtmlTableElement, HtmlTableRowElement,
    HtmlTableSectionElement, HtmlUListElement, HtmlVideoElement, Location, MediaDevices,
    MediaStream, MediaStreamConstraints, MediaTrackConstraints, Navigator, Request, Response,
    Window,
};

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn document() -> Document {
    let window: Window = window().expect("window");
    window.document().expect("document")
}

pub fn navigator() -> Navigator {
    let window: Window = window().expect("window");
    window.navigator()
}

pub fn now() -> f64 {
    let window: Window = window().expect("window");
    let performance = window.performance().expect("performace");
    performance.now()
}

pub fn wsurl() -> String {
    let window: Window = window().expect("window");
    let location: Location = window.location();
    let protocol: String = location.protocol().expect("protocol");
    let host: String = location.host().expect("host");
    let ws_protocol = if protocol == "https:" {
        "wss://"
    } else {
        "ws://"
    };
    format!("{}{}/ws/", ws_protocol, host)
}

pub fn start_button() -> HtmlButtonElement {
    document()
        .get_element_by_id("start_button")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlButtonElement")
}

pub fn start_menu() -> HtmlDivElement {
    document()
        .get_element_by_id("start_menu")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlDivElement")
}

pub fn search_input() -> HtmlInputElement {
    document()
        .get_element_by_id("search_input")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlInputElement")
}

pub fn search_suggestions() -> HtmlUListElement {
    document()
        .get_element_by_id("search_suggestions")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlUListElement")
}

pub fn search_suggestions_parent() -> HtmlDivElement {
    document()
        .get_element_by_id("search_suggestions_parent")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlDivElement")
}

pub fn div_class(class_name: &str) -> HtmlDivElement {
    let div: HtmlDivElement = document()
        .create_element("div")
        .expect("create_element")
        .dyn_into()
        .expect("HtmlDivElement");
    div.set_class_name(class_name);
    div
}

pub fn button_label(label: &str) -> HtmlButtonElement {
    let button: HtmlButtonElement = document()
        .create_element("button")
        .expect("create_element")
        .dyn_into()
        .expect("HtmlButtonElement");
    button.set_attribute("aria-label", label).unwrap();
    button
}

pub fn li() -> HtmlLiElement {
    document()
        .create_element("li")
        .expect("create_element")
        .dyn_into()
        .expect("HtmlLiElement")
}

pub fn search_table() -> HtmlTableElement {
    document()
        .get_element_by_id("search_table")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlTableElement")
}

pub fn search_results() -> HtmlTableSectionElement {
    document()
        .get_element_by_id("search_results")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlTableSectionElement")
}

pub fn remove_children(node: &HtmlElement) -> Result<(), JsValue> {
    while let Some(child) = node.first_child() {
        node.remove_child(&child)?;
    }
    Ok(())
}

pub fn tr() -> HtmlTableRowElement {
    document()
        .create_element("tr")
        .expect("create_element")
        .dyn_into()
        .expect("HtmlTableRowElement")
}

pub fn a() -> HtmlAnchorElement {
    document()
        .create_element("a")
        .expect("create_element")
        .dyn_into()
        .expect("HtmlAnchorElement")
}

pub fn video() -> HtmlVideoElement {
    document()
        .create_element("video")
        .expect("create_element")
        .dyn_into()
        .expect("HtmlVideoElement")
}

pub fn video_window(_id: &str, name: &str, video: HtmlVideoElement) -> HtmlDivElement {
    let title_bar_text = div_class("title-bar-text");
    title_bar_text.set_text_content(Some(name));
    let title_bar_controls = div_class("title-bar-controls");
    let minimize = button_label("Minimize");
    let maximize = button_label("Maximize");
    let close = button_label("Close");
    title_bar_controls
        .append_with_node_3(&minimize, &maximize, &close)
        .expect("title_bar_controls");
    let title_bar = div_class("title-bar");
    title_bar
        .append_with_node_2(&title_bar_text, &title_bar_controls)
        .expect("title_bar");
    let window_body = div_class("window-body");
    window_body.append_with_node_1(&video).expect("window_body");
    let root = div_class("window video");
    root.append_with_node_2(&title_bar, &window_body)
        .expect("root");
    root
}

pub fn local_video() -> HtmlVideoElement {
    document()
        .get_element_by_id("local_video")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlVideoElement")
}

pub fn remote_videos() -> HtmlDivElement {
    document()
        .get_element_by_id("remote_videos")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlDivElement")
}

pub fn nes_canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("nes_canvas")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlDivElement")
}

pub fn fps_p() -> HtmlParagraphElement {
    document()
        .get_element_by_id("fps")
        .expect("get_element_by_id")
        .dyn_into()
        .expect("HtmlParagraphElement")
}

fn media_constraints() -> MediaStreamConstraints {
    let mut width = ConstrainLongRange::new();
    width.ideal(320);
    let mut height = ConstrainLongRange::new();
    height.ideal(240);
    let mut video_constraints = MediaTrackConstraints::new();
    video_constraints.width(&width);
    video_constraints.height(&height);
    let mut media_constraints = MediaStreamConstraints::new();
    media_constraints.audio(&JsValue::from_bool(true));
    media_constraints.video(&video_constraints);
    media_constraints
}

pub async fn get_media_stream() -> Result<MediaStream, JsValue> {
    let navigator = navigator();
    let media_devices: MediaDevices = navigator.media_devices()?;
    let get_user_media_promise =
        media_devices.get_user_media_with_constraints(&media_constraints())?;
    let media_stream: MediaStream = JsFuture::from(get_user_media_promise).await?.dyn_into()?;
    Ok(media_stream)
}

pub async fn fetch_bytes_with_request(request: &Request) -> Result<Vec<u8>, JsValue> {
    let window: Window = window().expect("window");
    let resp_value = JsFuture::from(window.fetch_with_request(request)).await?;
    let resp: Response = resp_value.dyn_into().expect("response");
    let blob_value = JsFuture::from(resp.blob()?).await?;
    let blob: Blob = blob_value.dyn_into().expect("blob");
    let array_value = JsFuture::from(blob.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_value);
    let bytes = uint8_array.to_vec();
    Ok(bytes)
}

fn new_gamepad(gamepad: web_sys::Gamepad) -> nes::Gamepad {
    let id = gamepad.id();
    let index = gamepad.index();
    let axes = gamepad.axes().to_vec();
    let axes = axes.into_iter().map(|v| v.as_f64().unwrap()).collect();
    let buttons = gamepad.buttons().to_vec();
    let buttons: Vec<GamepadButton> = buttons.into_iter().map(|b| b.dyn_into().unwrap()).collect();
    let pressed = buttons.iter().map(|b| b.pressed()).collect();
    let touched = buttons.iter().map(|b| b.touched()).collect();
    let value = buttons.iter().map(|b| b.value()).collect();
    let timestamp = gamepad.timestamp();
    nes::Gamepad {
        id,
        index,
        pressed,
        touched,
        value,
        axes,
        timestamp,
    }
}

pub fn get_gamepads() -> Vec<nes::Gamepad> {
    let navigator = navigator();
    let gamepads = navigator.get_gamepads().unwrap();
    let gamepads = gamepads.to_vec();
    let mut res: Vec<nes::Gamepad> = vec![];
    for gamepad in gamepads {
        if let Ok(gamepad) = gamepad.dyn_into::<web_sys::Gamepad>() {
            let gamepad: nes::Gamepad = new_gamepad(gamepad);
            res.push(gamepad);
        }
    }
    res
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window: Window = window().expect("window");
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub trait MediaSupport {
    fn is_media_supported(&self) -> bool;
}

impl MediaSupport for ClientInfo {
    fn is_media_supported(&self) -> bool {
        let family: &str = self.user_agent.as_ref().expect("useragent").family.as_ref();
        family == "Chrome" || family == "Safari" || family == "Firefox" || family == "Edge"
    }
}

#[wasm_bindgen_test]
async fn test() {
    assert!(get_media_stream().await.is_ok());
}
