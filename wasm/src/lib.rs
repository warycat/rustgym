use seed::prelude::*;

mod init;
mod media;
mod message;
mod model;
mod update;
mod utils;
mod view;
mod websocket;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    App::start("app", init::init, update::update, view::view);
    Ok(())
}
