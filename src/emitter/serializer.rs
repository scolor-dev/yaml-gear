use wasm_bindgen::JsValue;
use crate::error::YamlError;

pub fn serialize(value: &JsValue, indent: usize) -> Result<String, YamlError> {
    if value.is_null() || value.is_undefined() {
        return Ok("null\n".to_string());
    }
    if let Some(b) = value.as_bool() {
        return Ok(format!("{}\n", b));
    }
    if let Some(f) = value.as_f64() {
        if f.fract() == 0.0 && f >= i64::MIN as f64 && f <= i64::MAX as f64 {
            return Ok(format!("{}\n", f as i64));
        }
        return Ok(format!("{}\n", f));
    }
    if let Some(s) = value.as_string() {
        return Ok(serialize_string(&s, indent));
    }
    if js_sys::Array::is_array(value) {
        return serialize_sequence(value, indent);
    }
    if value.is_object() {
        return serialize_mapping(value, indent);
    }
    Err(YamlError::StringifyError(format!("unsupported value: {:?}", value)))
}

fn serialize_string(s: &str, _indent: usize) -> String {
    if s.contains('\n') {
        let body = s.lines().collect::<Vec<_>>().join("\n  ");
        format!("|-\n  {}\n", body)
    } else if needs_quoting(s) {
        format!("'{}'\n", s.replace('\'', "''"))
    } else {
        format!("{}\n", s)
    }
}

fn serialize_sequence(value: &JsValue, indent: usize) -> Result<String, YamlError> {
    let arr = js_sys::Array::from(value);
    let pad = " ".repeat(indent);
    let mut out = String::new();
    for item in arr.iter() {
        if item.is_object() && !item.is_null() && !js_sys::Array::is_array(&item) {
            out.push_str(&format!("{}-\n", pad));
            out.push_str(&serialize(&item, indent + 2)?);
        } else if js_sys::Array::is_array(&item) {
            out.push_str(&format!("{}-\n", pad));
            out.push_str(&serialize(&item, indent + 2)?);
        } else {
            let s = serialize(&item, indent)?;
            let first = s.lines().next().unwrap_or("");
            out.push_str(&format!("{}- {}\n", pad, first));
        }
    }
    Ok(out)
}

fn serialize_mapping(value: &JsValue, indent: usize) -> Result<String, YamlError> {
    let obj = js_sys::Object::from(value.clone());
    let keys = js_sys::Object::keys(&obj);
    let pad = " ".repeat(indent);
    let mut out = String::new();
    for key in keys.iter() {
        let key_str = key.as_string().ok_or_else(|| {
            YamlError::StringifyError("object key must be a string".to_string())
        })?;
        let val = js_sys::Reflect::get(&obj, &key)
            .map_err(|_| YamlError::StringifyError(
                format!("failed to get key: {}", key_str)
            ))?;
        if (val.is_object() && !val.is_null()) || js_sys::Array::is_array(&val) {
            out.push_str(&format!("{}{}:\n", pad, key_str));
            out.push_str(&serialize(&val, indent + 2)?);
        } else {
            let s = serialize(&val, indent)?;
            let first = s.lines().next().unwrap_or("");
            out.push_str(&format!("{}{}: {}\n", pad, key_str, first));
        }
    }
    Ok(out)
}

fn needs_quoting(s: &str) -> bool {
    matches!(s, "true" | "false" | "null" | "~" | "yes" | "no")
        || s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '.')
        || s.contains([':', '#', '[', ']', '{', '}', ',', '&', '*', '?', '|', '>', '!', '%', '@', '`'])
}