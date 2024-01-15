use wasm_bindgen::prelude::*;
use web_sys::{Element, js_sys};
use js_sys::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn phoneNumber(element: Element);
}