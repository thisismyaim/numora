#[derive(Debug, Clone)]
pub struct ModeContext {
    pub modes: Vec<String>,
}

impl ModeContext {
    pub fn new(modes: Vec<String>) -> Self {
        Self { modes }
    }

    pub fn has_mode(&self, mode: &str) -> bool {
        self.modes.iter().any(|m| m == mode)
    }

    pub fn is_steps_enabled(&self) -> bool {
        self.has_mode("steps")
    }

    pub fn is_summary_enabled(&self) -> bool {
        self.has_mode("summary")
    }

    pub fn primary_execution_mode(&self) -> &str {
        self.modes
            .iter()
            .find(|mode| mode.as_str() == "calculator" || mode.as_str() == "physics")
            .map(|mode| mode.as_str())
            .unwrap_or("calculator")
    }
}
