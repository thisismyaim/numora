use crate::error::Numora;
use crate::modes::context::ModeContext;
use crate::modes::registry::ModeRegistry;

#[derive(Debug, Clone)]
pub struct ModePipeline {
    modes: Vec<String>,
}

impl ModePipeline {
    pub fn new(run_modes: Vec<String>) -> Result<Self, Numora> {
        let registry = ModeRegistry::new();

        let modes = Self::normalize(run_modes);

        Self::validate_known_modes(&modes, &registry)?;
        Self::validate_order(&modes, &registry)?;

        Ok(Self { modes })
    }

    pub fn modes(&self) -> &[String] {
        &self.modes
    }

    pub fn into_context(self) -> ModeContext {
        ModeContext::new(self.modes)
    }

    fn normalize(run_modes: Vec<String>) -> Vec<String> {
        if run_modes.is_empty() {
            return vec!["calculator".to_string()];
        }

        // V1 compatibility:
        //
        // @run steps
        //
        // means:
        //
        // @run calculator steps
        if run_modes.len() == 1 && run_modes[0] == "steps" {
            return vec!["calculator".to_string(), "steps".to_string()];
        }

        run_modes
    }

    fn validate_known_modes(modes: &[String], registry: &ModeRegistry) -> Result<(), Numora> {
        for mode in modes {
            if !registry.has(mode) {
                return Err(Numora::EvaluationError(format!(
                    "Unknown run mode '{}'. Available modes: {}",
                    mode,
                    registry.available_modes().join(", ")
                )));
            }
        }

        Ok(())
    }

    fn validate_order(modes: &[String], registry: &ModeRegistry) -> Result<(), Numora> {
        let mut output_mode_seen = false;

        for mode in modes {
            let info = registry
                .get(mode)
                .ok_or_else(|| Numora::EvaluationError(format!("Unknown run mode '{}'", mode)))?;

            if info.is_output() {
                output_mode_seen = true;
                continue;
            }

            if output_mode_seen && info.is_execution() {
                return Err(Numora::EvaluationError(format!(
                    "Invalid run mode order: '{}' cannot run after an output mode. Use execution modes first, then output modes. Example: '@run calculator steps'",
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

    #[test]
    fn calculator_steps_is_valid() {
        let pipeline = ModePipeline::new(vec!["calculator".to_string(), "steps".to_string()]);

        assert!(pipeline.is_ok());
    }

    #[test]
    fn physics_steps_is_valid() {
        let pipeline = ModePipeline::new(vec!["physics".to_string(), "steps".to_string()]);

        assert!(pipeline.is_ok());
    }

    #[test]
    fn solve_is_valid() {
        let pipeline = ModePipeline::new(vec!["solve".to_string()]);

        assert!(pipeline.is_ok());
    }

    #[test]
    fn solve_steps_is_valid() {
        let pipeline = ModePipeline::new(vec!["solve".to_string(), "steps".to_string()]);

        assert!(pipeline.is_ok());
    }

    #[test]
    fn steps_calculator_is_invalid() {
        let pipeline = ModePipeline::new(vec!["steps".to_string(), "calculator".to_string()]);

        assert!(pipeline.is_err());
    }

    #[test]
    fn summary_calculator_is_invalid() {
        let pipeline = ModePipeline::new(vec!["summary".to_string(), "calculator".to_string()]);

        assert!(pipeline.is_err());
    }

    #[test]
    fn unknown_mode_is_invalid() {
        let pipeline = ModePipeline::new(vec!["unknown".to_string()]);

        assert!(pipeline.is_err());
    }

    #[test]
    fn steps_normalizes_to_calculator_steps() {
        let pipeline = ModePipeline::new(vec!["steps".to_string()]).unwrap();

        assert_eq!(
            pipeline.modes(),
            &["calculator".to_string(), "steps".to_string()]
        );
    }

    #[test]
    fn empty_modes_default_to_calculator() {
        let pipeline = ModePipeline::new(vec![]).unwrap();

        assert_eq!(pipeline.modes(), &["calculator".to_string()]);
    }
}
