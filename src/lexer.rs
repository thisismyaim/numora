use crate::error::Numora;
use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Numora> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char() {
            match ch {
                '0'..='9' => {
                    let number = self.read_number()?;
                    tokens.push(Token::Number(number));
                }

                '.' => {
                    if self.next_char().map_or(false, |next| next.is_ascii_digit()) {
                        let number = self.read_number()?;
                        tokens.push(Token::Number(number));
                    } else {
                        return Err(Numora::LexerError("Unexpected '.'".to_string()));
                    }
                }

                'a'..='z' | 'A'..='Z' => {
                    let identifier = self.read_identifier();
                    tokens.push(Token::Identifier(identifier));
                }

                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }

                '-' | '_' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }

                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }

                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }

                '^' => {
                    tokens.push(Token::Power);
                    self.advance();
                }

                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }

                '=' => {
                    tokens.push(Token::Equal);
                    self.advance();
                }

                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }

                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }

                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }

                _ => {
                    return Err(Numora::LexerError(format!("Unknown character '{}'", ch)));
                }
            }
        }

        tokens.push(Token::End);
        Ok(tokens)
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn next_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_number(&mut self) -> Result<f64, Numora> {
        let mut number_text = String::new();
        let mut dot_count = 0;
        let mut digit_count = 0;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                digit_count += 1;
                number_text.push(ch);
                self.advance();
            } else if ch == '.' {
                dot_count += 1;

                if dot_count > 1 {
                    return Err(Numora::LexerError(
                        "Number has more than one decimal point".to_string(),
                    ));
                }

                number_text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if digit_count == 0 {
            return Err(Numora::LexerError(format!(
                "Invalid number '{}'",
                number_text
            )));
        }

        if number_text.ends_with('.') {
            return Err(Numora::LexerError(format!(
                "Invalid number '{}'",
                number_text
            )));
        }

        number_text
            .parse::<f64>()
            .map_err(|_| Numora::LexerError(format!("Invalid number '{}'", number_text)))
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }
}
