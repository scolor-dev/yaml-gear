mod error;
mod parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    parser::parse(input).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn stringify(value: JsValue) -> Result<JsValue, JsValue> {
    parser::stringify(value).map_err(|e| JsValue::from_str(&e.to_string()))
}

