#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeContext {
    normalized_modes: Vec<String>,
}

impl ModeContext {
    pub fn new(normalized_modes: Vec<String>) -> Self {
        Self { normalized_modes }
    }

    pub fn modes(&self) -> &[String] {
        &self.normalized_modes
    }

    pub fn execution_mode(&self) -> &str {
        self.normalized_modes
            .first()
            .map(|mode| mode.as_str())
            .unwrap_or("calculator")
    }

    pub fn has_mode(&self, mode: &str) -> bool {
        self.normalized_modes.iter().any(|m| m == mode)
    }

    pub fn wants_steps(&self) -> bool {
        self.has_mode("steps")
    }

    pub fn wants_summary(&self) -> bool {
        self.has_mode("summary")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_stores_normalized_modes() {
        let context = ModeContext::new(vec!["calculator".to_string(), "steps".to_string()]);

        assert_eq!(context.modes(), &["calculator", "steps"]);
    }

    #[test]
    fn execution_mode_is_first_mode() {
        let context = ModeContext::new(vec!["physics".to_string(), "steps".to_string()]);

        assert_eq!(context.execution_mode(), "physics");
    }

    #[test]
    fn empty_context_defaults_to_calculator() {
        let context = ModeContext::new(vec![]);

        assert_eq!(context.execution_mode(), "calculator");
    }

    #[test]
    fn detects_steps_mode() {
        let context = ModeContext::new(vec!["calculator".to_string(), "steps".to_string()]);

        assert!(context.wants_steps());
    }
}
