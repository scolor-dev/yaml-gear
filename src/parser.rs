use wasm_bindgen::prelude::*;
use crate::error::YamlError;

pub fn parse(input: &str) -> Result<JsValue, YamlError> {
    if input.trim().is_empty() {
        return Ok(JsValue::NULL);
    }

    // TODO: YAMLクレートを選定後に実装
    Err(YamlError::ParseError("not implemented yet".to_string()))
}

pub fn stringify(value: JsValue) -> Result<JsValue, YamlError> {
    if value.is_null() || value.is_undefined() {
        return Ok(JsValue::from_str("null\n"));
    }

    // TODO: YAMLクレートを選定後に実装
    Err(YamlError::StringifyError("not implemented yet".to_string()))
}