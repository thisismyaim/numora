use crate::environment::Environment;
use crate::error::Numora;
use crate::evaluator::evaluate;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Equation {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone)]
pub struct SolveResult {
    pub variable: String,
    pub value: f64,
}

pub fn solve_equation(
    equation: &Equation,
    solve_variable: &str,
    base_env: &Environment,
) -> Result<SolveResult, Numora> {
    validate_solve_variable(solve_variable)?;

    let left_ast = parse_expression(&equation.left)?;
    let right_ast = parse_expression(&equation.right)?;

    let function = |x: f64| -> Result<f64, Numora> {
        let mut env = base_env.clone();
        env.set(solve_variable.to_string(), crate::value::Value::number(x));

        let left_value = evaluate(&left_ast, &env)?;
        let right_value = evaluate(&right_ast, &env)?;

        if left_value.dim != right_value.dim {
            return Err(Numora::EvaluationError(format!(
                "Equation sides have different units: '{}' and '{}'",
                left_value.dim.format(),
                right_value.dim.format()
            )));
        }

        Ok(left_value.number - right_value.number)
    };

    let root = find_root(function)?;

    Ok(SolveResult {
        variable: solve_variable.to_string(),
        value: clean_near_zero(root),
    })
}

pub fn parse_equation(line: &str) -> Result<Equation, Numora> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();

    if parts.len() != 2 {
        return Err(Numora::ParserError(format!(
            "Expected equation like: x + 3 = 10, but found '{}'",
            line
        )));
    }

    let left = parts[0].trim();
    let right = parts[1].trim();

    if left.is_empty() {
        return Err(Numora::ParserError(
            "Equation left side cannot be empty".to_string(),
        ));
    }

    if right.is_empty() {
        return Err(Numora::ParserError(
            "Equation right side cannot be empty".to_string(),
        ));
    }

    Ok(Equation {
        left: left.to_string(),
        right: right.to_string(),
    })
}

fn parse_expression(source: &str) -> Result<crate::ast::Expr, Numora> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn validate_solve_variable(name: &str) -> Result<(), Numora> {
    let mut chars = name.chars();

    let Some(first) = chars.next() else {
        return Err(Numora::ParserError(
            "Solve variable cannot be empty".to_string(),
        ));
    };

    if !first.is_ascii_alphabetic() {
        return Err(Numora::ParserError(format!(
            "Solve variable '{}' must start with a letter",
            name
        )));
    }

    for ch in chars {
        if !ch.is_ascii_alphanumeric() && ch != '_' {
            return Err(Numora::ParserError(format!(
                "Solve variable '{}' contains invalid character '{}'",
                name, ch
            )));
        }
    }

    Ok(())
}

fn find_root<F>(function: F) -> Result<f64, Numora>
where
    F: Fn(f64) -> Result<f64, Numora>,
{
    let best_x = 0.0;
    let mut best_error = f64::INFINITY;

    let search_limits = [10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];

    for limit in search_limits {
        // First prefer positive domain.
        // This is better for radius, distance, side length, speed, etc.
        if let Some((left, right)) = find_bracket(&function, 0.0, limit, 2_000)? {
            return bisection(&function, left, right);
        }

        // Then try negative domain.
        if let Some((left, right)) = find_bracket(&function, -limit, 0.0, 2_000)? {
            return bisection(&function, left, right);
        }

        let candidate_error = function(best_x)?.abs();

        if candidate_error < best_error {
            best_error = candidate_error;
        }
    }

    if best_error < 1e-9 {
        return Ok(best_x);
    }

    Err(Numora::EvaluationError(
        "Could not solve equation numerically. Try giving a simpler equation or a closer form"
            .to_string(),
    ))
}

fn find_bracket<F>(
    function: &F,
    start: f64,
    end: f64,
    steps: usize,
) -> Result<Option<(f64, f64)>, Numora>
where
    F: Fn(f64) -> Result<f64, Numora>,
{
    let step_size = (end - start) / steps as f64;

    let mut previous_x = start;
    let mut previous_y = function(previous_x)?;

    if previous_y.abs() < 1e-9 {
        return Ok(Some((previous_x, previous_x)));
    }

    for index in 1..=steps {
        let current_x = start + index as f64 * step_size;
        let current_y = function(current_x)?;

        if current_y.abs() < 1e-9 {
            return Ok(Some((current_x, current_x)));
        }

        if previous_y.signum() != current_y.signum() {
            return Ok(Some((previous_x, current_x)));
        }

        previous_x = current_x;
        previous_y = current_y;
    }

    Ok(None)
}

fn bisection<F>(function: &F, mut left: f64, mut right: f64) -> Result<f64, Numora>
where
    F: Fn(f64) -> Result<f64, Numora>,
{
    if (left - right).abs() < 1e-12 {
        return Ok(left);
    }

    let mut left_y = function(left)?;

    for _ in 0..200 {
        let middle = (left + right) / 2.0;
        let middle_y = function(middle)?;

        if middle_y.abs() < 1e-10 {
            return Ok(middle);
        }

        if left_y.signum() == middle_y.signum() {
            left = middle;
            left_y = middle_y;
        } else {
            right = middle;
        }
    }

    Ok((left + right) / 2.0)
}

fn clean_near_zero(value: f64) -> f64 {
    if value.abs() < 1e-10 {
        0.0
    } else {
        value
    }
}
