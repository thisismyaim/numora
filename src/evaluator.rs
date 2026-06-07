use crate::ast::{BinaryOperator, Expr, UnaryOperator};
use crate::builtins::{call_builtin_function, evaluate_builtin_symbol};
use crate::environment::Environment;
use crate::error::Numora;
use crate::value::Value;

pub fn evaluate(expr: &Expr, env: &Environment) -> Result<Value, Numora> {
    match expr {
        Expr::Number(value) => Ok(Value::number(*value)),

        Expr::Quantity { number, unit } => Value::quantity(*number, unit),

        Expr::Symbol { name } => evaluate_symbol(name, env),

        Expr::FunctionCall { name, arguments } => evaluate_function_call(name, arguments, env),

        Expr::Unary {
            operator,
            expression,
        } => {
            let value = evaluate(expression, env)?;

            match operator {
                UnaryOperator::Negative => Ok(value.negative()),
            }
        }

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left_value = evaluate(left, env)?;
            let right_value = evaluate(right, env)?;

            match operator {
                BinaryOperator::Add => left_value.add(right_value),

                BinaryOperator::Subtract => left_value.subtract(right_value),

                BinaryOperator::Multiply => Ok(left_value.multiply(right_value)),

                BinaryOperator::Divide => left_value.divide(right_value),

                BinaryOperator::Power => left_value.power(right_value),
            }
        }
    }
}

fn evaluate_symbol(name: &str, env: &Environment) -> Result<Value, Numora> {
    if let Some(value) = env.get(name) {
        return Ok(value);
    }

    let builtin_value = evaluate_builtin_symbol(name)?;
    Ok(Value::number(builtin_value))
}

fn evaluate_function_call(
    name: &str,
    arguments: &[Expr],
    env: &Environment,
) -> Result<Value, Numora> {
    if name == "sqrt" {
        return evaluate_sqrt(arguments, env);
    }

    let mut values = Vec::with_capacity(arguments.len());

    for argument in arguments {
        let value = evaluate(argument, env)?;

        if !value.is_dimensionless() {
            return Err(Numora::EvaluationError(format!(
                "{}() only supports values without units for now, but got '{}'",
                name,
                value.format()
            )));
        }

        values.push(value.number);
    }

    let result = call_builtin_function(name, &values)?;
    Ok(Value::number(result))
}

fn evaluate_sqrt(arguments: &[Expr], env: &Environment) -> Result<Value, Numora> {
    if arguments.len() != 1 {
        return Err(Numora::EvaluationError(format!(
            "sqrt() needs exactly 1 argument, but got {}",
            arguments.len()
        )));
    }

    let value = evaluate(&arguments[0], env)?;

    if value.number < 0.0 {
        return Err(Numora::EvaluationError(
            "sqrt() cannot take a negative number in real-number mode".to_string(),
        ));
    }

    let new_dim = value.dim.sqrt()?;

    Ok(Value {
        number: value.number.sqrt(),
        dim: new_dim,
    })
}
