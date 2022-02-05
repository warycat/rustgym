use std::boxed::Box;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::{HtmlButtonElement, HtmlDivElement, MouseEvent};

#[derive(Debug, Clone)]
pub struct Desktop {
    _start_menu: HtmlDivElement,
    _start_button: HtmlButtonElement,
}

impl Desktop {
    pub fn new(_start_menu: HtmlDivElement, _start_button: HtmlButtonElement) -> Desktop {
        let start_menu_clone = _start_menu.clone();
        let click_cb = Closure::wrap(Box::new(move |_e: MouseEvent| {
            let class_list = start_menu_clone.class_list();
            class_list.toggle("hidden").unwrap();
        }) as Box<dyn FnMut(_)>);
        _start_button.set_onclick(Some(click_cb.as_ref().unchecked_ref()));
        click_cb.forget();
        Desktop {
            _start_menu,
            _start_button,
        }
    }
}
