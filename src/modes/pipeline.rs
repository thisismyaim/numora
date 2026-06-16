use crate::error::Numora;
use crate::modes::{ModeContext, ModeRegistry};

pub struct ModePipeline {
    requested_modes: Vec<String>,
}

impl ModePipeline {
    pub fn new(requested_modes: Vec<String>) -> Self {
        Self { requested_modes }
    }

    pub fn build_context(&self) -> Result<ModeContext, Numora> {
        let normalized_modes = self.normalize_modes()?;
        self.validate_modes(&normalized_modes)?;

        Ok(ModeContext::new(normalized_modes))
    }

    fn normalize_modes(&self) -> Result<Vec<String>, Numora> {
        let modes: Vec<String> = self
            .requested_modes
            .iter()
            .map(|mode| mode.trim().to_lowercase())
            .filter(|mode| !mode.is_empty())
            .collect();

        if modes.is_empty() {
            return Ok(vec!["calculator".to_string()]);
        }

        // V1 compatibility:
        // @run steps behaves like @run calculator steps
        if modes == ["steps"] {
            return Ok(vec!["calculator".to_string(), "steps".to_string()]);
        }

        Ok(modes)
    }

    fn validate_modes(&self, modes: &[String]) -> Result<(), Numora> {
        for mode in modes {
            if !ModeRegistry::is_known(mode) {
                return Err(Numora::EvaluationError(format!(
                    "Unknown run mode: {}",
                    mode
                )));
            }
        }

        let execution_count = modes
            .iter()
            .filter(|mode| ModeRegistry::is_execution(mode))
            .count();

        if execution_count == 0 {
            return Err(Numora::EvaluationError(
                "A run pipeline must start with an execution mode: calculator, physics, or solve"
                    .to_string(),
            ));
        }

        if execution_count > 1 {
            return Err(Numora::EvaluationError(
                "A run pipeline can only contain one execution mode".to_string(),
            ));
        }

        let first_mode = modes
            .first()
            .map(|mode| mode.as_str())
            .unwrap_or("calculator");

        if !ModeRegistry::is_execution(first_mode) {
            return Err(Numora::EvaluationError(format!(
                "Invalid mode order: '{}' cannot run before an execution mode",
                first_mode
            )));
        }

        for mode in modes.iter().skip(1) {
            if ModeRegistry::is_execution(mode) {
                return Err(Numora::EvaluationError(format!(
                    "Invalid mode order: execution mode '{}' must be first",
                    mode
                )));
            }
        }

        // For now, output modes are terminal.
        // Valid:
        //   calculator steps
        //   physics steps
        // Invalid:
        //   steps calculator
        //   summary calculator
        if let Some((index, mode)) = modes
            .iter()
            .enumerate()
            .find(|(_, mode)| ModeRegistry::is_output(mode))
        {
            if index != modes.len() - 1 {
                return Err(Numora::EvaluationError(format!(
                    "Invalid mode order: output mode '{}' must be last",
                    mode
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(input: &[&str], expected: &[&str]) {
        let pipeline = ModePipeline::new(input.iter().map(|mode| mode.to_string()).collect());

        let context = pipeline.build_context().unwrap();

        assert_eq!(
            context.modes(),
            expected
                .iter()
                .map(|mode| mode.to_string())
                .collect::<Vec<_>>()
        );
    }

    fn assert_invalid(input: &[&str]) {
        let pipeline = ModePipeline::new(input.iter().map(|mode| mode.to_string()).collect());

        assert!(pipeline.build_context().is_err());
    }

    #[test]
    fn empty_modes_default_to_calculator() {
        assert_valid(&[], &["calculator"]);
    }

    #[test]
    fn calculator_steps_is_valid() {
        assert_valid(&["calculator", "steps"], &["calculator", "steps"]);
    }

    #[test]
    fn physics_steps_is_valid() {
        assert_valid(&["physics", "steps"], &["physics", "steps"]);
    }

    #[test]
    fn solve_is_valid() {
        assert_valid(&["solve"], &["solve"]);
    }

    #[test]
    fn solve_steps_is_valid() {
        assert_valid(&["solve", "steps"], &["solve", "steps"]);
    }

    #[test]
    fn steps_normalizes_to_calculator_steps() {
        assert_valid(&["steps"], &["calculator", "steps"]);
    }

    #[test]
    fn steps_calculator_is_invalid() {
        assert_invalid(&["steps", "calculator"]);
    }

    #[test]
    fn summary_calculator_is_invalid() {
        assert_invalid(&["summary", "calculator"]);
    }

    #[test]
    fn unknown_mode_is_invalid() {
        assert_invalid(&["unknown"]);
    }
}
