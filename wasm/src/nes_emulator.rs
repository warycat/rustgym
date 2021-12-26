use wasm_bindgen::*;
use wasm_bindgen_test::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct NesEmulator {
    canvas: HtmlCanvasElement,
    filename: String,
    md5: String,
}

impl NesEmulator {
    pub fn new(canvas: HtmlCanvasElement, filename: String, md5: String) -> NesEmulator {
        console_log!(".....");
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        ctx.set_fill_style(&JsValue::from_str("blue"));
        ctx.fill_rect(10., 10., 100., 100.);
        NesEmulator {
            canvas,
            filename,
            md5,
        }
    }
}
