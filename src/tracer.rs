use crate::ast::{BinaryOperator, Expr, UnaryOperator};
use crate::environment::Environment;
use crate::error::Numora;
use crate::evaluator::evaluate;
use crate::format::format_number;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct TraceResult {
    pub value: Value,
    pub steps: Vec<String>,
}

pub fn trace_assignment(
    target_name: &str,
    expr: &Expr,
    env: &Environment,
) -> Result<TraceResult, Numora> {
    let mut steps = Vec::new();

    let original = format!("{} = {}", target_name, expr_to_string(expr));
    steps.push(original);

    let substituted_expr = substitute_symbols(expr, env);

    let substituted_text = format!("{} = {}", target_name, expr_to_string(&substituted_expr));

    if steps.last() != Some(&substituted_text) {
        steps.push(substituted_text);
    }

    let mut current_expr = substituted_expr;

    loop {
        let next_expr = simplify_one_step(&current_expr)?;

        let current_text = expr_to_string(&current_expr);
        let next_text = expr_to_string(&next_expr);

        if current_text == next_text {
            break;
        }

        steps.push(format!("{} = {}", target_name, next_text));
        current_expr = next_expr;
    }

    let value = evaluate(&current_expr, env)?;

    Ok(TraceResult { value, steps })
}

fn substitute_symbols(expr: &Expr, env: &Environment) -> Expr {
    match expr {
        Expr::Number(value) => Expr::Number(*value),

        Expr::Quantity { number, unit } => Expr::Quantity {
            number: *number,
            unit: unit.clone(),
        },

        Expr::Symbol { name } => {
            if let Some(value) = env.get(name) {
                if value.is_dimensionless() {
                    Expr::Number(value.number)
                } else {
                    Expr::Symbol { name: name.clone() }
                }
            } else {
                Expr::Symbol { name: name.clone() }
            }
        }

        Expr::FunctionCall { name, arguments } => {
            let new_arguments = arguments
                .iter()
                .map(|arg| substitute_symbols(arg, env))
                .collect();

            Expr::FunctionCall {
                name: name.clone(),
                arguments: new_arguments,
            }
        }

        Expr::Unary {
            operator,
            expression,
        } => Expr::Unary {
            operator: operator.clone(),
            expression: Box::new(substitute_symbols(expression, env)),
        },

        Expr::Binary {
            left,
            operator,
            right,
        } => Expr::Binary {
            left: Box::new(substitute_symbols(left, env)),
            operator: operator.clone(),
            right: Box::new(substitute_symbols(right, env)),
        },
    }
}

fn simplify_one_step(expr: &Expr) -> Result<Expr, Numora> {
    match expr {
        Expr::Number(_) | Expr::Quantity { .. } | Expr::Symbol { .. } => Ok(expr.clone()),

        Expr::Unary {
            operator,
            expression,
        } => {
            let simplified_inner = simplify_one_step(expression)?;

            if expr_to_string(expression) != expr_to_string(&simplified_inner) {
                return Ok(Expr::Unary {
                    operator: operator.clone(),
                    expression: Box::new(simplified_inner),
                });
            }

            match (&operator, simplified_inner) {
                (UnaryOperator::Negative, Expr::Number(value)) => Ok(Expr::Number(-value)),

                (_, other) => Ok(Expr::Unary {
                    operator: operator.clone(),
                    expression: Box::new(other),
                }),
            }
        }

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let simplified_left = simplify_one_step(left)?;

            if expr_to_string(left) != expr_to_string(&simplified_left) {
                return Ok(Expr::Binary {
                    left: Box::new(simplified_left),
                    operator: operator.clone(),
                    right: right.clone(),
                });
            }

            let simplified_right = simplify_one_step(right)?;

            if expr_to_string(right) != expr_to_string(&simplified_right) {
                return Ok(Expr::Binary {
                    left: left.clone(),
                    operator: operator.clone(),
                    right: Box::new(simplified_right),
                });
            }

            match (&**left, operator, &**right) {
                (Expr::Number(left_value), op, Expr::Number(right_value)) => {
                    let value = match op {
                        BinaryOperator::Add => left_value + right_value,
                        BinaryOperator::Subtract => left_value - right_value,
                        BinaryOperator::Multiply => left_value * right_value,
                        BinaryOperator::Divide => {
                            if *right_value == 0.0 {
                                return Err(Numora::EvaluationError(
                                    "Cannot divide by zero".to_string(),
                                ));
                            }

                            left_value / right_value
                        }
                        BinaryOperator::Power => left_value.powf(*right_value),
                    };

                    Ok(Expr::Number(value))
                }

                _ => Ok(expr.clone()),
            }
        }

        Expr::FunctionCall { name, arguments } => {
            let mut new_arguments = Vec::with_capacity(arguments.len());
            let mut changed = false;

            for argument in arguments {
                if !changed {
                    let simplified = simplify_one_step(argument)?;

                    if expr_to_string(argument) != expr_to_string(&simplified) {
                        changed = true;
                        new_arguments.push(simplified);
                    } else {
                        new_arguments.push(argument.clone());
                    }
                } else {
                    new_arguments.push(argument.clone());
                }
            }

            if changed {
                return Ok(Expr::FunctionCall {
                    name: name.clone(),
                    arguments: new_arguments,
                });
            }

            let temp_env = Environment::new();
            let value = evaluate(expr, &temp_env)?;

            if value.is_dimensionless() {
                Ok(Expr::Number(value.number))
            } else {
                Ok(expr.clone())
            }
        }
    }
}

pub fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Number(value) => format_number(*value),

        Expr::Quantity { number, unit } => {
            format!("{} {}", format_number(*number), unit)
        }

        Expr::Symbol { name } => name.clone(),

        Expr::FunctionCall { name, arguments } => {
            let args = arguments
                .iter()
                .map(expr_to_string)
                .collect::<Vec<String>>()
                .join(", ");

            format!("{}({})", name, args)
        }

        Expr::Unary {
            operator,
            expression,
        } => match operator {
            UnaryOperator::Negative => format!("-{}", expr_to_string(expression)),
        },

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let op = match operator {
                BinaryOperator::Add => "+",
                BinaryOperator::Subtract => "-",
                BinaryOperator::Multiply => "*",
                BinaryOperator::Divide => "/",
                BinaryOperator::Power => "^",
            };

            format!(
                "({} {} {})",
                expr_to_string(left),
                op,
                expr_to_string(right)
            )
        }
    }
}
