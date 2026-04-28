#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    // --- 構造 ---
    Colon,
    Dash,
    Newline,
    Indent(usize),

    // --- スカラー値 ---
    Scalar(&'a str),
    SingleQuoted(String),   // エスケープ処理で新規生成が必要なのでStringのまま
    DoubleQuoted(String),   // 同上

    // --- ドキュメント ---
    DocStart,
    DocEnd,

    // --- その他 ---
    Eof,
}

#[derive(Debug, Clone)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub line: usize,
    pub col: usize,
}