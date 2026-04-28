#![allow(dead_code)]

mod error;
pub mod lexer;
mod parser;
mod emitter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    parser::parse(input)
        .map_err(|e: crate::error::YamlError| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn parse_all(input: &str) -> Result<JsValue, JsValue> {
    parser::parse_all(input)
        .map_err(|e: crate::error::YamlError| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn stringify(value: JsValue) -> Result<JsValue, JsValue> {
    emitter::stringify(value)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}