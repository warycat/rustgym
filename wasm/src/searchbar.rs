use crate::client::*;
use crate::utils::*;
use rustgym_msg::*;
use std::boxed::Box;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::{
    HtmlDivElement, HtmlInputElement, HtmlTableElement, HtmlTableRowElement,
    HtmlTableSectionElement, InputEvent, KeyboardEvent, MouseEvent,
};

#[derive(Debug, Clone)]
pub struct SearchRow {
    div: HtmlDivElement,
}

impl SearchRow {
    fn new(div: HtmlDivElement, text: String) -> Self {
        let div_clone = div.clone();
        div.set_text_content(Some(&text));
        div.set_class_name("pl-10 pr-4 py-2");
        let click_cb = Closure::wrap(Box::new(move |_e: MouseEvent| {
            let text = div_clone.text_content().expect("text");
            get_client().on_query_text(text).expect("on_query_text");
        }) as Box<dyn FnMut(_)>);
        div.set_onclick(Some(click_cb.as_ref().unchecked_ref()));
        click_cb.forget();
        SearchRow { div }
    }
}

#[derive(Debug, Clone)]
pub struct ResultRow {
    tr: HtmlTableRowElement,
}

impl ResultRow {
    fn new(query_result: QueryResult) -> Result<Self, JsValue> {
        let tr = tr();
        let id = tr.insert_cell()?;
        id.set_inner_text(&query_result.id.to_string());
        let title = tr.insert_cell()?;
        let link = a();
        link.set_href(&query_result.href);
        link.set_text(&query_result.title)?;
        title.append_with_node_1(&link)?;
        let from = tr.insert_cell()?;
        from.set_inner_text(&query_result.from);
        Ok(ResultRow { tr })
    }
}

#[derive(Debug, Clone)]
pub struct SearchBar {
    search_input: HtmlInputElement,
    search_suggestions: HtmlDivElement,
    search_table: HtmlTableElement,
    search_results: HtmlTableSectionElement,
}

impl SearchBar {
    pub fn new(
        search_input: HtmlInputElement,
        search_suggestions: HtmlDivElement,
        search_table: HtmlTableElement,
        search_results: HtmlTableSectionElement,
    ) -> Self {
        let search_input_clone = search_input.clone();
        let input_cb = Closure::wrap(Box::new(move |_e: InputEvent| {
            let search_text = search_input_clone.value();
            get_client()
                .on_search_text_change(search_text)
                .expect("on_search_text_change");
        }) as Box<dyn FnMut(_)>);
        search_input.set_oninput(Some(input_cb.as_ref().unchecked_ref()));
        input_cb.forget();

        let search_input_clone = search_input.clone();
        let keydown_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let search_text = search_input_clone.value();
            if e.key_code() == 13 {
                get_client()
                    .on_query_text(search_text)
                    .expect("on_query_text");
            }
        }) as Box<dyn FnMut(_)>);
        search_input.set_onkeydown(Some(keydown_cb.as_ref().unchecked_ref()));
        keydown_cb.forget();

        SearchBar {
            search_input,
            search_suggestions,
            search_table,
            search_results,
        }
    }

    pub fn close_search_suggestions(&self) -> Result<(), JsValue> {
        self.search_suggestions.class_list().add_1("empty")
    }

    pub fn update_search_suggestions(
        &self,
        search_suggestions: Vec<String>,
    ) -> Result<(), JsValue> {
        remove_children(&self.search_suggestions)?;
        if search_suggestions.is_empty() {
            self.search_suggestions.class_list().add_1("empty")?;
        } else {
            self.search_suggestions.class_list().remove_1("empty")?;
            for suggestion in search_suggestions {
                let row = SearchRow::new(div(), suggestion);
                self.search_suggestions.append_with_node_1(&row.div)?;
            }
        }
        Ok(())
    }

    pub fn update_query_results(&self, query_results: Vec<QueryResult>) -> Result<(), JsValue> {
        if query_results.is_empty() {
            self.search_table.class_list().add_1("empty")?;
        } else {
            self.search_table.class_list().remove_1("empty")?;
            for query_result in query_results {
                let row = ResultRow::new(query_result)?;
                self.search_results.append_with_node_1(&row.tr)?;
            }
        }
        Ok(())
    }
}
