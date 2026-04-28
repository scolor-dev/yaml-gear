pub mod serializer;
use wasm_bindgen::prelude::*;
use crate::error::YamlError;
use serializer::serialize;

pub fn stringify(value: JsValue) -> Result<JsValue, YamlError> {
    let output = serialize(&value, 0)?;
    Ok(JsValue::from_str(&output))
}
