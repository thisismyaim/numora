use crate::config::LanguageConfig;
use crate::error::Numora;
use crate::program::run_math_program;

pub struct Runtime {
    config: LanguageConfig,
}

impl Runtime {
    pub fn new(config: LanguageConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, source: &str) -> Result<String, Numora> {
        self.validate_feature_access(source)?;

        if self.looks_like_math_program(source) {
            return run_math_program(source);
        }

        run_math_program(source)
    }

    fn validate_feature_access(&self, source: &str) -> Result<(), Numora> {
        if source.contains("unit:") && !self.config.units_enabled {
            return Err(Numora::EvaluationError(
                "Units feature is disabled".to_string(),
            ));
        }

        if source.contains("@run ide") && !self.config.ide_api_enabled {
            return Err(Numora::EvaluationError(
                "IDE API feature is disabled".to_string(),
            ));
        }

        Ok(())
    }

    fn looks_like_math_program(&self, source: &str) -> bool {
        source.contains("@run")
            || source.contains("given:")
            || source.contains("formula:")
            || source.contains("equation:")
            || source.contains("find:")
            || source.contains("solve:")
            || source.contains("input:")
            || source.contains("unit:")
    }
}
