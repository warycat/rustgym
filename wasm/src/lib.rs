use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

macro_rules! console_dbg {
    ($($arg:tt)*) => {
        console_log!("{} {}: {}", file!(), line!(), &format_args!($($arg)*))
    };
}

mod client;
mod media;
mod nes_emulator;
mod pc;
mod searchbar;
mod utils;

use client::*;
use media::MediaClient;
use nes_emulator::NesEmulator;
use searchbar::SearchBar;
use utils::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    set_panic_hook();
    get_client();
    let searchbar = SearchBar::new(
        search_input(),
        search_suggestions(),
        search_table(),
        search_results(),
    );
    set_searchbar(searchbar);
    let media_client = MediaClient::new(local_video(), remote_videos());
    set_media_client(media_client);
    Ok(())
}

#[wasm_bindgen]
pub async fn start_nes(filename: String, md5: String) -> Result<(), JsValue> {
    console_log!("nes_start {:?} {:?}", filename, md5);
    let nes_emulator = NesEmulator::new(nes_canvas(), filename, md5);
    Ok(())
}
