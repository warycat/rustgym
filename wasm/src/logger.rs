use log::Level::*;
use log::{Metadata, Record};
use rustgym_consts::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str, level: &str, message: &str);
}

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LOG_LEVEL
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log(
                &format!(
                    "%c{}%c - {}::{} >> {}",
                    record.level(),
                    record.file_static().unwrap(),
                    record.line().unwrap(),
                    record.args(),
                ),
                match record.level() {
                    Error => "color: red;",
                    Warn => "color: yellow;",
                    Info => "color: green;",
                    Debug => "color: blue;",
                    Trace => "color: magenta;",
                },
                "color: none;",
            );
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), JsValue> {
    log::set_logger(&LOGGER)
        .map(|_| log::set_max_level(LOG_LEVEL_FILTER))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
