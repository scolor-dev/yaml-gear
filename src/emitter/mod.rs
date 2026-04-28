// emitter/mod.rs
pub mod serializer;
use wasm_bindgen::prelude::*;
use crate::error::YamlError;
use serializer::serialize;

pub fn stringify(value: JsValue) -> Result<JsValue, YamlError> {
    let output = serialize(&value, 0)?;
    Ok(JsValue::from_str(&output))
}

#[cfg(test)]
mod tests {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_stringify_object() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &JsValue::from_str("Alice"),
        ).unwrap();
        let result = super::stringify(obj.into()).unwrap();
        assert_eq!(result.as_string().unwrap(), "name: Alice\n");
    }

    #[wasm_bindgen_test]
    fn test_stringify_null() {
        let result = super::stringify(JsValue::NULL).unwrap();
        assert_eq!(result.as_string().unwrap(), "null\n");
    }
}