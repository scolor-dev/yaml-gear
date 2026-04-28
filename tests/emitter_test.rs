use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;
use yaml_gear::stringify;

#[wasm_bindgen_test]
fn test_stringify_object() {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("name"),
        &JsValue::from_str("Alice"),
    ).unwrap();
    let result = stringify(obj.into()).unwrap();
    assert_eq!(result.as_string().unwrap(), "name: Alice\n");
}

#[wasm_bindgen_test]
fn test_stringify_null() {
    let result = stringify(JsValue::NULL).unwrap();
    assert_eq!(result.as_string().unwrap(), "null\n");
}