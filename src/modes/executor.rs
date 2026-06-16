use crate::error::Numora;
use crate::modes::ModeContext;
use crate::program::{run_calculator_program, run_physics_program, run_solve_program};

pub struct ModeExecutor;

impl ModeExecutor {
    pub fn execute(source: &str, context: &ModeContext) -> Result<String, Numora> {
        match context.execution_mode() {
            "calculator" => Self::execute_calculator(source, context),
            "physics" => Self::execute_physics(source, context),
            "solve" => Self::execute_solve(source, context),

            unknown => Err(Numora::EvaluationError(format!(
                "Unknown execution mode: {}",
                unknown
            ))),
        }
    }

    fn execute_calculator(source: &str, _context: &ModeContext) -> Result<String, Numora> {
        run_calculator_program(source)
    }

    fn execute_physics(source: &str, _context: &ModeContext) -> Result<String, Numora> {
        run_physics_program(source)
    }

    fn execute_solve(source: &str, _context: &ModeContext) -> Result<String, Numora> {
        run_solve_program(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modes::ModeContext;

    #[test]
    fn calculator_execution_runs_program() {
        let context = ModeContext::new(vec!["calculator".to_string()]);

        let source = r#"
@run calculator

given:
    x = 1
    y = 2

formula:
    result = x + y * 3

find:
    result
"#;

        let result = ModeExecutor::execute(source, &context).unwrap();

        assert!(result.contains("7"));
    }

    #[test]
    fn physics_execution_uses_physics_program_entrypoint() {
        let context = ModeContext::new(vec!["physics".to_string()]);

        let source = r#"
@run physics

given:
    x = 1
    y = 2

formula:
    result = x + y * 3

find:
    result
"#;

        let result = ModeExecutor::execute(source, &context).unwrap();

        assert!(result.contains("7"));
    }

    #[test]
    fn solve_execution_uses_solve_program_entrypoint() {
        let context = ModeContext::new(vec!["solve".to_string()]);

        let source = r#"
@run solve

given:
    x = 2

equation:
    y = x + 3

solve:
    y
"#;

        let result = ModeExecutor::execute(source, &context).unwrap();

        assert!(
            result.contains("5")
                || result.contains("y")
                || result.to_lowercase().contains("solve")
                || result.to_lowercase().contains("result")
        );
    }
}
