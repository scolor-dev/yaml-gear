use std::fmt;

#[derive(Debug)]
pub enum YamlError {
    ParseError(String),
    StringifyError(String),
}

impl fmt::Display for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YamlError::ParseError(msg) => write!(f, "parse error: {}", msg),
            YamlError::StringifyError(msg) => write!(f, "stringify error: {}", msg),
        }
    }
}

impl std::error::Error for YamlError {}
