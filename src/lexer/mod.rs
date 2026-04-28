pub mod token;

use token::{SpannedToken, Token};
use crate::error::YamlError;

pub const MAX_DEPTH: usize = 32;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<SpannedToken<'a>, YamlError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }
        Some(self.next_token())
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0, line: 1, col: 1 }
    }

    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken<'a>>, YamlError> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            let token = self.next_token()?;
            tokens.push(token);
        }
        tokens.push(self.make_token(Token::Eof));
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<SpannedToken<'a>, YamlError> {
        self.skip_comment();

        if self.is_eof() {
            return Ok(self.make_token(Token::Eof));
        }

        match self.current_char() {
            '\n' => {
                self.advance();
                Ok(self.make_token(Token::Newline))
            }
            '-' if self.peek_is("---") => {
                self.advance_by(3);
                Ok(self.make_token(Token::DocStart))
            }
            '.' if self.peek_is("...") => {
                self.advance_by(3);
                Ok(self.make_token(Token::DocEnd))
            }
            '-' if self.next_is_whitespace() => {
                self.advance();
                Ok(self.make_token(Token::Dash))
            }
            ':' if self.next_is_whitespace() => {
                self.advance();
                Ok(self.make_token(Token::Colon))
            }
            ' ' if self.col == 1 => {
                let count = self.count_indent();
                if count > MAX_DEPTH * 2 {
                    return Err(YamlError::ParseError(
                        format!("exceeded max nesting depth of {}", MAX_DEPTH)
                    ));
                }
                Ok(self.make_token(Token::Indent(count)))
            }
            ' ' | '\t' => {
                self.advance();
                self.next_token()
            }
            '\'' => self.read_single_quoted(),
            '"'  => self.read_double_quoted(),
            _    => self.read_scalar(),
        }
    }

    fn read_single_quoted(&mut self) -> Result<SpannedToken<'a>, YamlError> {
        self.advance();
        let mut s = String::new();
        loop {
            if self.is_eof() {
                return Err(YamlError::ParseError(
                    format!("unterminated single-quoted string at line {}", self.line)
                ));
            }
            match self.current_char() {
                '\'' => {
                    self.advance();
                    if self.current_char() == '\'' {
                        s.push('\'');
                        self.advance();
                    } else {
                        break;
                    }
                }
                c => { s.push(c); self.advance(); }
            }
        }
        Ok(self.make_token(Token::SingleQuoted(s)))
    }

    fn read_double_quoted(&mut self) -> Result<SpannedToken<'a>, YamlError> {
        self.advance();
        let mut s = String::new();
        loop {
            if self.is_eof() {
                return Err(YamlError::ParseError(
                    format!("unterminated double-quoted string at line {}", self.line)
                ));
            }
            match self.current_char() {
                '"' => { self.advance(); break; }
                '\\' => {
                    self.advance();
                    match self.current_char() {
                        'n'  => { s.push('\n'); self.advance(); }
                        't'  => { s.push('\t'); self.advance(); }
                        'r'  => { s.push('\r'); self.advance(); }
                        '\\' => { s.push('\\'); self.advance(); }
                        '"'  => { s.push('"');  self.advance(); }
                        '0'  => { s.push('\0'); self.advance(); }
                        c => return Err(YamlError::ParseError(
                            format!("unknown escape sequence: \\{} at line {}", c, self.line)
                        )),
                    }
                }
                c => { s.push(c); self.advance(); }
            }
        }
        Ok(self.make_token(Token::DoubleQuoted(s)))
    }

    fn read_scalar(&mut self) -> Result<SpannedToken<'a>, YamlError> {
        let start = self.pos;
        while !self.is_eof() {
            match self.current_char() {
                '\n' => break,
                ':' if self.next_is_whitespace() => break,
                '#' if self.prev_is_whitespace() => break,
                _ => self.advance(),
            }
        }
        let raw = &self.input[start..self.pos];
        let trimmed_start = start + (raw.len() - raw.trim_start().len());
        let trimmed_end   = self.pos - (raw.len() - raw.trim_end().len());

        if trimmed_start >= trimmed_end {
            return Err(YamlError::ParseError(
                format!("unexpected character at line {}, col {}", self.line, self.col)
            ));
        }

        let value = &self.input[trimmed_start..trimmed_end];
        Ok(self.make_token(Token::Scalar(value)))
    }

    // --- ヘルパー ---

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn peek_is(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn next_is_whitespace(&self) -> bool {
        let mut chars = self.input[self.pos..].chars();
        chars.next();
        matches!(chars.next(), Some(' ') | Some('\n') | Some('\t') | None)
    }

    fn prev_is_whitespace(&self) -> bool {
        if self.pos == 0 { return true; }
        matches!(self.input[..self.pos].chars().last(), Some(' ') | Some('\t'))
    }

    fn advance(&mut self) {
        if let Some(c) = self.input[self.pos..].chars().next() {
            self.pos += c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }

    fn advance_by(&mut self, n: usize) {
        for _ in 0..n { self.advance(); }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn skip_comment(&mut self) {
        if self.current_char() == '#' && self.prev_is_whitespace() {
            while !self.is_eof() && self.current_char() != '\n' {
                self.advance();
            }
        }
    }

    fn count_indent(&mut self) -> usize {
        let mut count = 0;
        while self.current_char() == ' ' {
            self.advance();
            count += 1;
        }
        count
    }

    fn make_token(&self, token: Token<'a>) -> SpannedToken<'a> {
        SpannedToken { token, line: self.line, col: self.col }
    }
}
