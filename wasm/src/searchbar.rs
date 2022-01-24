use crate::client::*;
use crate::utils::*;
use rustgym_msg::*;
use std::boxed::Box;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::{
    HtmlDivElement, HtmlInputElement, HtmlLiElement, HtmlTableElement, HtmlTableRowElement,
    HtmlTableSectionElement, HtmlUListElement, InputEvent, KeyboardEvent, MouseEvent,
};

#[derive(Debug, Clone)]
pub struct SearchRow {
    li: HtmlLiElement,
}

impl SearchRow {
    fn new(li: HtmlLiElement, text: String) -> Self {
        let li_clone = li.clone();
        li.set_text_content(Some(&text));
        li.set_class_name("search-row");
        let click_cb = Closure::wrap(Box::new(move |_e: MouseEvent| {
            let text = li_clone.text_content().expect("text");
            get_client().on_query_text(text).expect("on_query_text");
        }) as Box<dyn FnMut(_)>);
        li.set_onclick(Some(click_cb.as_ref().unchecked_ref()));
        click_cb.forget();
        SearchRow { li }
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
        id.set_inner_text(&query_result.id);
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
    search_suggestions: HtmlUListElement,
    search_suggestions_parent: HtmlDivElement,
    search_table: HtmlTableElement,
    search_results: HtmlTableSectionElement,
}

impl SearchBar {
    pub fn new(
        search_input: HtmlInputElement,
        search_suggestions: HtmlUListElement,
        search_suggestions_parent: HtmlDivElement,
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
            search_suggestions_parent,
            search_table,
            search_results,
        }
    }

    pub fn close_search_suggestions(&self) -> Result<(), JsValue> {
        self.search_suggestions.class_list().add_1("hidden")
    }

    pub fn update_search_input(&self, text: &str) {
        self.search_input.set_value(text);
    }

    pub fn update_search_suggestions(
        &self,
        search_suggestions: Vec<String>,
    ) -> Result<(), JsValue> {
        remove_children(&self.search_suggestions)?;
        if search_suggestions.is_empty() {
            self.search_suggestions_parent
                .class_list()
                .add_1("hidden")?;
        } else {
            self.search_suggestions_parent
                .class_list()
                .remove_1("hidden")?;
            for suggestion in search_suggestions {
                let row = SearchRow::new(li(), suggestion);
                self.search_suggestions.append_with_node_1(&row.li)?;
            }
        }
        Ok(())
    }

    pub fn update_query_results(&self, query_results: Vec<QueryResult>) -> Result<(), JsValue> {
        if query_results.is_empty() {
            self.search_table.class_list().add_1("hidden")?;
        } else {
            self.search_table.class_list().remove_1("hidden")?;
            for query_result in query_results {
                let row = ResultRow::new(query_result)?;
                self.search_results.append_with_node_1(&row.tr)?;
            }
        }
        Ok(())
    }
}
