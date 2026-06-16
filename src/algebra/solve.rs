use crate::error::Numora;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearEquation {
    pub variable: String,
    pub coefficient: f64,
    pub constant: f64,
    pub equals: f64,
}

impl LinearEquation {
    pub fn new(variable: impl Into<String>, coefficient: f64, constant: f64, equals: f64) -> Self {
        Self {
            variable: variable.into(),
            coefficient,
            constant,
            equals,
        }
    }
}

/// Solves a simple equation in this form:
///
/// coefficient * x + constant = equals
///
/// Example:
/// 2x + 3 = 11
/// x = (11 - 3) / 2
/// x = 4
pub fn solve_linear_equation(equation: LinearEquation) -> Result<f64, Numora> {
    if equation.coefficient == 0.0 {
        return Err(Numora::EvaluationError(format!(
            "Cannot solve linear equation for '{}': coefficient cannot be zero",
            equation.variable
        )));
    }

    Ok((equation.equals - equation.constant) / equation.coefficient)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_simple_linear_equation() {
        let equation = LinearEquation::new("x", 2.0, 3.0, 11.0);

        let result = solve_linear_equation(equation).unwrap();

        assert_eq!(result, 4.0);
    }

    #[test]
    fn rejects_zero_coefficient() {
        let equation = LinearEquation::new("x", 0.0, 3.0, 11.0);

        let result = solve_linear_equation(equation);

        assert!(result.is_err());
    }
}
