use crate::ast::{BinaryOperator, Expr, UnaryOperator};
use crate::error::Numora;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, Numora> {
        let expr = self.parse_add_subtract()?;

        if !self.is_at_end() {
            return Err(Numora::ParserError(format!(
                "Unexpected token: {:?}",
                self.current_token()
            )));
        }

        Ok(expr)
    }

    fn parse_add_subtract(&mut self) -> Result<Expr, Numora> {
        let mut expr = self.parse_multiply_divide()?;

        loop {
            match self.current_token() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiply_divide()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: BinaryOperator::Add,
                        right: Box::new(right),
                    };
                }

                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiply_divide()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_multiply_divide(&mut self) -> Result<Expr, Numora> {
        let mut expr = self.parse_power()?;

        loop {
            match self.current_token() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_power()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(right),
                    };
                }

                Token::Slash => {
                    self.advance();
                    let right = self.parse_power()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: BinaryOperator::Divide,
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<Expr, Numora> {
        let mut expr = self.parse_unary()?;

        while matches!(self.current_token(), Token::Power) {
            self.advance();
            let right = self.parse_unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, Numora> {
        match self.current_token() {
            Token::Minus => {
                self.advance();

                let expression = self.parse_unary()?;

                Ok(Expr::Unary {
                    operator: UnaryOperator::Negative,
                    expression: Box::new(expression),
                })
            }

            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, Numora> {
        match self.current_token().clone() {
            Token::Number(value) => {
                self.advance();

                if let Token::Identifier(unit_name) = self.current_token().clone() {
                    if is_unit_name(&unit_name) {
                        self.advance();

                        return Ok(Expr::Quantity {
                            number: value,
                            unit: unit_name,
                        });
                    }
                }

                Ok(Expr::Number(value))
            }

            Token::Identifier(name) => {
                self.advance();

                if matches!(self.current_token(), Token::LeftParen) {
                    self.parse_function_call(name)
                } else {
                    Ok(Expr::Symbol { name })
                }
            }

            Token::LeftParen => {
                self.advance();

                let expr = self.parse_add_subtract()?;

                if !matches!(self.current_token(), Token::RightParen) {
                    return Err(Numora::ParserError(
                        "Expected ')' after expression".to_string(),
                    ));
                }

                self.advance();
                Ok(expr)
            }

            token => Err(Numora::ParserError(format!(
                "Expected number, symbol, function, or '(' but found {:?}",
                token
            ))),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<Expr, Numora> {
        self.expect_left_paren()?;

        let mut arguments = Vec::new();

        if matches!(self.current_token(), Token::RightParen) {
            self.advance();
            return Ok(Expr::FunctionCall { name, arguments });
        }

        loop {
            let argument = self.parse_add_subtract()?;
            arguments.push(argument);

            match self.current_token() {
                Token::Comma => {
                    self.advance();
                }

                Token::RightParen => {
                    self.advance();
                    break;
                }

                token => {
                    return Err(Numora::ParserError(format!(
                        "Expected ',' or ')' in function call but found {:?}",
                        token
                    )));
                }
            }
        }

        Ok(Expr::FunctionCall { name, arguments })
    }

    fn expect_left_paren(&mut self) -> Result<(), Numora> {
        if matches!(self.current_token(), Token::LeftParen) {
            self.advance();
            Ok(())
        } else {
            Err(Numora::ParserError(
                "Expected '(' after function name".to_string(),
            ))
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::End)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::End)
    }
}

fn is_unit_name(name: &str) -> bool {
    matches!(name, "m" | "cm" | "km" | "s" | "kg" | "g")
}
