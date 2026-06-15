use std::collections::HashMap;

use crate::modes::mode::ModeInfo;

#[derive(Debug, Clone)]
pub struct ModeRegistry {
    modes: HashMap<String, ModeInfo>,
}

impl ModeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            modes: HashMap::new(),
        };

        // Execution / domain modes
        registry.register_execution("calculator");
        registry.register_execution("physics");

        // Output / explanation modes
        registry.register_output("steps");
        registry.register_output("summary");

        registry
    }

    pub fn get(&self, name: &str) -> Option<&ModeInfo> {
        self.modes.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.modes.contains_key(name)
    }

    pub fn available_modes(&self) -> Vec<String> {
        let mut modes = self.modes.keys().cloned().collect::<Vec<_>>();
        modes.sort();
        modes
    }

    fn register_execution(&mut self, name: &str) {
        self.modes
            .insert(name.to_string(), ModeInfo::execution(name));
    }

    fn register_output(&mut self, name: &str) {
        self.modes.insert(name.to_string(), ModeInfo::output(name));
    }
}

impl Default for ModeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
