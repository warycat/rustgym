use rustgym_msg::ClientInfo;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{
    window, Document, HtmlAnchorElement, HtmlDivElement, HtmlElement, HtmlInputElement,
    HtmlTableElement, HtmlTableRowElement, HtmlTableSectionElement, HtmlVideoElement, Location,
    MediaDevices, MediaStream, MediaStreamConstraints, Navigator, Window,
};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
struct JsSide {
    ideal: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
struct JsVideo {
    width: JsSide,
    height: JsSide,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
struct JsConstraints {
    video: JsVideo,
    audio: bool,
}

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

pub fn search_input() -> HtmlInputElement {
    document()
        .get_element_by_id("search_input")
        .expect("get_element_by_id")
        .dyn_into::<HtmlInputElement>()
        .expect("HtmlInputElement")
}

pub fn search_suggestions() -> HtmlDivElement {
    document()
        .get_element_by_id("search_suggestions")
        .expect("get_element_by_id")
        .dyn_into::<HtmlDivElement>()
        .expect("HtmlDivElement")
}

pub fn div() -> HtmlDivElement {
    document()
        .create_element("div")
        .expect("create_element")
        .dyn_into::<HtmlDivElement>()
        .expect("HtmlDivElement")
}

pub fn search_table() -> HtmlTableElement {
    document()
        .get_element_by_id("search_table")
        .expect("get_element_by_id")
        .dyn_into::<HtmlTableElement>()
        .expect("HtmlTableElement")
}

pub fn search_results() -> HtmlTableSectionElement {
    document()
        .get_element_by_id("search_results")
        .expect("get_element_by_id")
        .dyn_into::<HtmlTableSectionElement>()
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
        .dyn_into::<HtmlTableRowElement>()
        .expect("HtmlTableRowElement")
}

pub fn a() -> HtmlAnchorElement {
    document()
        .create_element("a")
        .expect("create_element")
        .dyn_into::<HtmlAnchorElement>()
        .expect("HtmlAnchorElement")
}

pub fn video() -> HtmlVideoElement {
    document()
        .create_element("video")
        .expect("create_element")
        .dyn_into::<HtmlVideoElement>()
        .expect("HtmlVideoElement")
}

pub fn local_video() -> HtmlVideoElement {
    document()
        .get_element_by_id("local_video")
        .expect("get_element_by_id")
        .dyn_into::<HtmlVideoElement>()
        .expect("HtmlVideoElement")
}

pub fn remote_videos() -> HtmlDivElement {
    document()
        .get_element_by_id("remote_videos")
        .expect("get_element_by_id")
        .dyn_into::<HtmlDivElement>()
        .expect("HtmlDivElement")
}

pub async fn get_media_stream() -> Result<MediaStream, JsValue> {
    let navigator = navigator();
    let media_devices: MediaDevices = navigator.media_devices()?;
    let js_constraints = JsConstraints {
        video: JsVideo {
            width: JsSide { ideal: 320 },
            height: JsSide { ideal: 240 },
        },
        audio: true,
    };

    let constraints =
        MediaStreamConstraints::from(JsValue::from_serde(&js_constraints).expect("js_constrants"));
    let get_user_media_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream: MediaStream = JsFuture::from(get_user_media_promise).await?.dyn_into()?;
    Ok(media_stream)
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
    assert_eq!(get_media_stream().await.is_ok(), true);
}
