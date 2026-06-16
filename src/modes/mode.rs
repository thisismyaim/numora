#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeCategory {
    Execution,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    pub name: &'static str,
    pub category: ModeCategory,
}

impl Mode {
    pub const fn new(name: &'static str, category: ModeCategory) -> Self {
        Self { name, category }
    }

    pub fn is_execution(&self) -> bool {
        self.category == ModeCategory::Execution
    }

    pub fn is_output(&self) -> bool {
        self.category == ModeCategory::Output
    }
}
