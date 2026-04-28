#[derive(Debug, PartialEq)]
pub enum YamlValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Sequence(Vec<YamlValue>),
    Mapping(Vec<(String, YamlValue)>),
}