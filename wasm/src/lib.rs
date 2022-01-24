use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

macro_rules! console_dbg {
    ($($arg:tt)*) => {
        console_log!("{} {}: {}", file!(), line!(), &format_args!($($arg)*))
    };
}

mod client;
mod desktop;
mod media;
mod nes_emulator;
mod pc;
mod searchbar;
mod utils;

use client::*;
use media::MediaClient;
#[macro_use]
extern crate derive_new;

use nes_emulator::NesEmulator;
use searchbar::SearchBar;
use utils::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    set_panic_hook();
    get_client();
    let media_client = MediaClient::new(local_video(), remote_videos());
    set_media_client(media_client);
    Ok(())
}

#[wasm_bindgen]
pub async fn start_find() -> Result<(), JsValue> {
    console_log!("start_find");
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
    console_log!("start_nes {:?} {:?}", filename, md5);
    let mut nes_emulator = NesEmulator::new(nes_canvas(), filename, md5);
    nes_emulator.load_rom().await?;
    nes_emulator.run();
    Ok(())
}
