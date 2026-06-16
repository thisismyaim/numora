use crate::algebra::AlgebraExpr;
use crate::error::Numora;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
}

pub fn parse_algebra_expression(source: &str) -> Result<AlgebraExpr, Numora> {
    let tokens = tokenize(source)?;
    let mut parser = AlgebraParser::new(tokens);
    let expr = parser.parse_expression()?;

    if !parser.is_at_end() {
        return Err(Numora::ParserError(format!(
            "Unexpected token after algebra expression: {:?}",
            parser.peek()
        )));
    }

    Ok(expr)
}

struct AlgebraParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AlgebraParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse_expression(&mut self) -> Result<AlgebraExpr, Numora> {
        self.parse_add_subtract()
    }

    fn parse_add_subtract(&mut self) -> Result<AlgebraExpr, Numora> {
        let mut expr = self.parse_multiply_divide()?;

        while self.match_token(&TokenKind::Plus) || self.match_token(&TokenKind::Minus) {
            let operator = self.previous().cloned();

            let right = self.parse_multiply_divide()?;

            match operator {
                Some(Token::Plus) => {
                    expr = AlgebraExpr::add(expr, right);
                }
                Some(Token::Minus) => {
                    expr = AlgebraExpr::subtract(expr, right);
                }
                _ => {
                    return Err(Numora::ParserError("Expected + or - operator".to_string()));
                }
            }
        }

        Ok(expr)
    }

    fn parse_multiply_divide(&mut self) -> Result<AlgebraExpr, Numora> {
        let mut expr = self.parse_power()?;

        while self.match_token(&TokenKind::Star) || self.match_token(&TokenKind::Slash) {
            let operator = self.previous().cloned();

            let right = self.parse_power()?;

            match operator {
                Some(Token::Star) => {
                    expr = AlgebraExpr::multiply(expr, right);
                }
                Some(Token::Slash) => {
                    expr = AlgebraExpr::divide(expr, right);
                }
                _ => {
                    return Err(Numora::ParserError("Expected * or / operator".to_string()));
                }
            }
        }

        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<AlgebraExpr, Numora> {
        let left = self.parse_unary()?;

        if self.match_token(&TokenKind::Caret) {
            // Right-associative:
            // x ^ 2 ^ 3 means x ^ (2 ^ 3)
            let right = self.parse_power()?;
            return Ok(AlgebraExpr::power(left, right));
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<AlgebraExpr, Numora> {
        if self.match_token(&TokenKind::Minus) {
            let right = self.parse_unary()?;

            return Ok(AlgebraExpr::subtract(AlgebraExpr::number(0.0), right));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<AlgebraExpr, Numora> {
        if let Some(token) = self.advance().cloned() {
            match token {
                Token::Number(value) => Ok(AlgebraExpr::number(value)),
                Token::Identifier(name) => Ok(AlgebraExpr::variable(name)),
                Token::LeftParen => {
                    let expr = self.parse_expression()?;

                    if !self.match_token(&TokenKind::RightParen) {
                        return Err(Numora::ParserError(
                            "Expected ')' after algebra expression".to_string(),
                        ));
                    }

                    Ok(expr)
                }
                other => Err(Numora::ParserError(format!(
                    "Unexpected token in algebra expression: {:?}",
                    other
                ))),
            }
        } else {
            Err(Numora::ParserError(
                "Unexpected end of algebra expression".to_string(),
            ))
        }
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }

        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        match (kind, self.peek()) {
            (TokenKind::Plus, Some(Token::Plus)) => true,
            (TokenKind::Minus, Some(Token::Minus)) => true,
            (TokenKind::Star, Some(Token::Star)) => true,
            (TokenKind::Slash, Some(Token::Slash)) => true,
            (TokenKind::Caret, Some(Token::Caret)) => true,
            (TokenKind::RightParen, Some(Token::RightParen)) => true,
            _ => false,
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        if self.current == 0 {
            return None;
        }

        self.tokens.get(self.current - 1)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    RightParen,
}

fn tokenize(source: &str) -> Result<Vec<Token>, Numora> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let ch = chars[index];

        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                index += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                index += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                index += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                index += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                index += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                index += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                index += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                index += 1;
            }
            '0'..='9' | '.' => {
                let start = index;

                index += 1;

                while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '.')
                {
                    index += 1;
                }

                let number_text: String = chars[start..index].iter().collect();

                let value = number_text.parse::<f64>().map_err(|_| {
                    Numora::ParserError(format!(
                        "Invalid number in algebra expression: {}",
                        number_text
                    ))
                })?;

                tokens.push(Token::Number(value));
            }
            _ if ch.is_ascii_alphabetic() || ch == '_' => {
                let start = index;

                index += 1;

                while index < chars.len()
                    && (chars[index].is_ascii_alphanumeric() || chars[index] == '_')
                {
                    index += 1;
                }

                let identifier: String = chars[start..index].iter().collect();
                tokens.push(Token::Identifier(identifier));
            }
            _ => {
                return Err(Numora::ParserError(format!(
                    "Unexpected character in algebra expression: '{}'",
                    ch
                )));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::simplify_expression;

    #[test]
    fn parses_number() {
        let expr = parse_algebra_expression("5").unwrap();

        assert_eq!(expr.to_string(), "5");
    }

    #[test]
    fn parses_variable() {
        let expr = parse_algebra_expression("x").unwrap();

        assert_eq!(expr.to_string(), "x");
    }

    #[test]
    fn parses_addition() {
        let expr = parse_algebra_expression("x + 2").unwrap();

        assert_eq!(expr.to_string(), "(x + 2)");
    }

    #[test]
    fn respects_operator_precedence() {
        let expr = parse_algebra_expression("x + 2 * y").unwrap();

        assert_eq!(expr.to_string(), "(x + (2 * y))");
    }

    #[test]
    fn parses_parentheses() {
        let expr = parse_algebra_expression("(x + 2) * y").unwrap();

        assert_eq!(expr.to_string(), "((x + 2) * y)");
    }

    #[test]
    fn parses_power() {
        let expr = parse_algebra_expression("x ^ 2").unwrap();

        assert_eq!(expr.to_string(), "(x ^ 2)");
    }

    #[test]
    fn power_is_right_associative() {
        let expr = parse_algebra_expression("x ^ 2 ^ 3").unwrap();

        assert_eq!(expr.to_string(), "(x ^ (2 ^ 3))");
    }

    #[test]
    fn parses_unary_minus() {
        let expr = parse_algebra_expression("-x").unwrap();

        assert_eq!(expr.to_string(), "(0 - x)");
    }

    #[test]
    fn parsed_expression_can_be_simplified() {
        let expr = parse_algebra_expression("x + 0").unwrap();
        let simplified = simplify_expression(expr);

        assert_eq!(simplified.to_string(), "x");
    }

    #[test]
    fn rejects_invalid_character() {
        let result = parse_algebra_expression("x @ 2");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unclosed_parentheses() {
        let result = parse_algebra_expression("(x + 2");

        assert!(result.is_err());
    }
}
