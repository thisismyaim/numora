use std::fmt;

#[derive(Debug, Clone)]
pub enum Numora {
    LexerError(String),
    ParserError(String),
    EvaluationError(String),
}

impl fmt::Display for Numora {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Numora::LexerError(msg) => {
                write!(f, "Lexer Error: {}", msg)
            }
            Numora::ParserError(msg) => {
                write!(f, "Parser Error: {}", msg)
            }
            Numora::EvaluationError(msg) => {
                write!(f, "Evaluation Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for Numora {}
