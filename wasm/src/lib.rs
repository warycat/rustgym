#[macro_use]
extern crate derive_new;

mod client;
mod desktop;
mod gamepad;
mod logger;
mod media;
mod nes_emulator;
mod pc;
mod searchbar;
mod shaders;
mod utils;

pub use client::*;
pub use media::*;
pub use nes_emulator::*;
pub use searchbar::*;
pub use shaders::*;
pub use utils::*;

use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() -> Result<(), JsValue> {
    logger::init()?;
    set_panic_hook();
    get_client();
    let media_client = MediaClient::new(local_video(), remote_videos());
    set_media_client(media_client);
    Ok(())
}

#[wasm_bindgen]
pub async fn start_find() -> Result<(), JsValue> {
    info!("start_find");
    let searchbar = SearchBar::new(
        search_input(),
        search_suggestions(),
        search_suggestions_parent(),
        search_table(),
        search_results(),
    );
    set_searchbar(searchbar);
    Ok(())
}

#[wasm_bindgen]
pub async fn start_nes(filename: String, md5: String) -> Result<(), JsValue> {
    info!("start_nes {:?} {:?}", filename, md5);
    let mut nes_emulator = NesEmulator::new(nes_canvas(), filename, md5);
    nes_emulator.load_rom().await?;
    nes_emulator.run();
    Ok(())
}
