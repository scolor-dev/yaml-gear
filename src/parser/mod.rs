use wasm_bindgen::prelude::*;
use crate::lexer::token::{SpannedToken, Token};
use crate::error::YamlError;
use crate::lexer::MAX_DEPTH;
use crate::lexer::Lexer;

// ===== 公開API =====

pub fn parse(input: &str) -> Result<JsValue, YamlError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

pub fn parse_all(input: &str) -> Result<JsValue, YamlError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let arr = js_sys::Array::new();
    loop {
        parser.skip_newlines();
        if parser.is_eof() { break; }
        // DocStart（---）をスキップ
        if matches!(parser.current_token(), Token::DocStart) {
            parser.advance();
            parser.skip_newlines();
        }
        if parser.is_eof() { break; }
        let doc = parser.parse_value(0)?;
        arr.push(&doc);
        parser.skip_newlines();
        // DocEnd（...）をスキップ
        if matches!(parser.current_token(), Token::DocEnd) {
            parser.advance();
        }
    }
    Ok(arr.into())
}

// ===== Parser =====

struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<SpannedToken<'a>>,
    next: Option<SpannedToken<'a>>,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next().and_then(|r| r.ok());
        let next    = lexer.next().and_then(|r| r.ok());
        Parser { lexer, current, next, depth: 0 }
    }

    fn parse(&mut self) -> Result<JsValue, YamlError> {
        self.skip_newlines();
        self.parse_value(0)
    }

    fn enter(&mut self) -> Result<(), YamlError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            Err(YamlError::ParseError(
                format!("exceeded max nesting depth of {}", MAX_DEPTH)
            ))
        } else {
            Ok(())
        }
    }

    fn leave(&mut self) {
        if self.depth > 0 { self.depth -= 1; }
    }

    fn parse_value(&mut self, indent: usize) -> Result<JsValue, YamlError> {
        match self.current_token() {
            Token::Dash => self.parse_sequence(indent),
            Token::Scalar(_) => {
                if self.peek_is_colon() {
                    self.parse_mapping(indent)
                } else {
                    self.parse_scalar()
                }
            }
            Token::SingleQuoted(s) => {
                let value = JsValue::from_str(s);
                self.advance();
                Ok(value)
            }
            Token::DoubleQuoted(s) => {
                let value = JsValue::from_str(s);
                self.advance();
                Ok(value)
            }
            Token::DocStart => {
                self.advance();
                self.skip_newlines();
                self.parse_value(indent)
            }
            Token::Eof => Ok(JsValue::NULL),
            token => Err(YamlError::ParseError(
                format!("unexpected token: {:?}", token)
            )),
        }
    }

    fn parse_mapping(&mut self, indent: usize) -> Result<JsValue, YamlError> {
        self.enter()?;
        let obj = js_sys::Object::new();

        while !self.is_eof() {
            if let Token::Indent(n) = self.current_token() {
                if *n < indent { break; }
                self.advance();
            }

            let key = match self.current_token() {
                Token::Scalar(s) => {
                    let k = JsValue::from_str(s);
                    self.advance();
                    k
                }
                _ => break,
            };

            self.expect(Token::Colon)?;
            self.skip_newlines();

            let value = if matches!(self.current_token(), Token::Newline | Token::Eof) {
                self.skip_newlines();
                let child_indent = self.current_indent();
                self.parse_value(child_indent)?
            } else {
                self.parse_value(indent)?
            };

            js_sys::Reflect::set(&obj, &key, &value)
                .map_err(|_| YamlError::ParseError(
                    format!("failed to set key: {:?}", key)
                ))?;

            self.skip_newlines();
        }

        self.leave();
        Ok(obj.into())
    }

    fn parse_sequence(&mut self, indent: usize) -> Result<JsValue, YamlError> {
        self.enter()?;
        let arr = js_sys::Array::new();

        while matches!(self.current_token(), Token::Dash) {
            self.advance();
            self.skip_newlines();
            let value = self.parse_value(indent + 2)?;
            arr.push(&value);
            self.skip_newlines();

            if let Token::Indent(n) = self.current_token() {
                if *n < indent { break; }
            }
        }

        self.leave();
        Ok(arr.into())
    }

    fn parse_scalar(&mut self) -> Result<JsValue, YamlError> {
        if let Token::Scalar(s) = self.current_token() {
            let value = interpret_scalar(s);
            self.advance();
            Ok(value)
        } else {
            Err(YamlError::ParseError("expected scalar".to_string()))
        }
    }

    // --- ヘルパー ---

    fn current_token(&self) -> &Token<'a> {
        self.current
            .as_ref()
            .map(|t| &t.token)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.current = self.next.take();
        self.next = self.lexer.next().and_then(|r| r.ok());
    }

    fn is_eof(&self) -> bool {
        matches!(self.current_token(), Token::Eof)
    }

    fn peek_is_colon(&self) -> bool {
        self.next
            .as_ref()
            .map(|t| matches!(t.token, Token::Colon))
            .unwrap_or(false)
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current_token(), Token::Newline | Token::Indent(_)) {
            self.advance();
        }
    }

    fn current_indent(&self) -> usize {
        if let Token::Indent(n) = self.current_token() {
            *n
        } else {
            0
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), YamlError> {
        if std::mem::discriminant(self.current_token())
            == std::mem::discriminant(&expected)
        {
            self.advance();
            Ok(())
        } else {
            Err(YamlError::ParseError(format!(
                "expected {:?}, got {:?}", expected, self.current_token()
            )))
        }
    }
}

// ===== スカラー型解釈 =====

fn interpret_scalar(s: &str) -> JsValue {
    match s {
        "null" | "~" | "" => JsValue::NULL,
        "true" | "yes"    => JsValue::TRUE,
        "false" | "no"    => JsValue::FALSE,
        _ => {
            if let Ok(i) = s.parse::<i64>() { return JsValue::from_f64(i as f64); }
            if let Ok(f) = s.parse::<f64>() { return JsValue::from_f64(f); }
            JsValue::from_str(s)
        }
    }
}
