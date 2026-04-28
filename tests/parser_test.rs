use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;
use yaml_gear::parse;
use yaml_gear::parse_all;

#[wasm_bindgen_test]
fn test_mapping() {
    let v = parse("name: Alice\nage: 30\n").unwrap();
    let name = js_sys::Reflect::get(&v, &JsValue::from_str("name")).unwrap();
    let age = js_sys::Reflect::get(&v, &JsValue::from_str("age")).unwrap();
    assert_eq!(name.as_string().unwrap(), "Alice");
    assert_eq!(age.as_f64().unwrap(), 30.0);
}

#[wasm_bindgen_test]
fn test_bool_and_null() {
    let v = parse("a: true\nb: null\n").unwrap();
    let a = js_sys::Reflect::get(&v, &JsValue::from_str("a")).unwrap();
    let b = js_sys::Reflect::get(&v, &JsValue::from_str("b")).unwrap();
    assert_eq!(a.as_bool().unwrap(), true);
    assert!(b.is_null());
}

#[wasm_bindgen_test]
fn test_sequence() {
    let v = parse("- foo\n- bar\n").unwrap();
    let arr = js_sys::Array::from(&v);
    assert_eq!(arr.get(0).as_string().unwrap(), "foo");
    assert_eq!(arr.get(1).as_string().unwrap(), "bar");
}

#[wasm_bindgen_test]
fn test_max_depth_exceeded() {
    let mut input = String::new();
    for i in 0..33 {
        input.push_str(&format!("{}key{}:\n", "  ".repeat(i), i));
    }
    assert!(parse(&input).is_err());
}

#[wasm_bindgen_test]
fn test_multi_document() {
    let input = "---\nfoo: 1\n---\nbar: 2\n";
    let v = parse_all(input).unwrap();
    let arr = js_sys::Array::from(&v);
    assert_eq!(arr.length(), 2);
    let doc1 = arr.get(0);
    let doc2 = arr.get(1);
    let foo = js_sys::Reflect::get(&doc1, &JsValue::from_str("foo")).unwrap();
    let bar = js_sys::Reflect::get(&doc2, &JsValue::from_str("bar")).unwrap();
    assert_eq!(foo.as_f64().unwrap(), 1.0);
    assert_eq!(bar.as_f64().unwrap(), 2.0);
}