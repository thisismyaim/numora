#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Comma,
    Equal,

    LeftParen,
    RightParen,

    End,
}
