use yaml_gear::lexer::Lexer;
use yaml_gear::lexer::token::Token;

fn tokenize(input: &str) -> Vec<Token<'_>> {
    Lexer::new(input)
        .tokenize()
        .unwrap()
        .into_iter()
        .map(|t| t.token)
        .collect()
}

#[test]
fn test_simple_mapping() {
    let tokens = tokenize("name: Alice\n");
    assert_eq!(tokens, vec![
        Token::Scalar("name"),
        Token::Colon,
        Token::Scalar("Alice"),
        Token::Newline,
        Token::Eof,
    ]);
}

#[test]
fn test_sequence() {
    let tokens = tokenize("- foo\n- bar\n");
    assert_eq!(tokens, vec![
        Token::Dash,
        Token::Scalar("foo"),
        Token::Newline,
        Token::Dash,
        Token::Scalar("bar"),
        Token::Newline,
        Token::Eof,
    ]);
}

#[test]
fn test_comment_skipped() {
    let tokens = tokenize("name: Alice # comment\n");
    assert_eq!(tokens, vec![
        Token::Scalar("name"),
        Token::Colon,
        Token::Scalar("Alice"),
        Token::Newline,
        Token::Eof,
    ]);
}

#[test]
fn test_max_depth_exceeded() {
    use yaml_gear::lexer::MAX_DEPTH;
    let indent = " ".repeat(MAX_DEPTH * 2 + 1);
    let input = format!("{}key: value\n", indent);
    let result = Lexer::new(&input).tokenize();
    assert!(result.is_err());
}