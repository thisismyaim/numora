#[derive(Debug, Clone)]
pub struct RunContext {
    pub result_name: Option<String>,
    pub result_display: Option<String>,
    pub trace: Vec<String>,
    pub warnings: Vec<String>,
}

impl RunContext {
    pub fn new() -> Self {
        Self {
            result_name: None,
            result_display: None,
            trace: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn has_result(&self) -> bool {
        self.result_display.is_some()
    }
}
