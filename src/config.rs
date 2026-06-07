#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub calculator_enabled: bool,
    pub variables_enabled: bool,
    pub steps_enabled: bool,
    pub equations_enabled: bool,
    pub units_enabled: bool,
    pub ide_api_enabled: bool,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            calculator_enabled: true,
            variables_enabled: true,
            steps_enabled: true,
            equations_enabled: true,
            units_enabled: true,

            // Future phase
            ide_api_enabled: false,
        }
    }
}
