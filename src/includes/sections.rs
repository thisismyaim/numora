use crate::error::Numora;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MathSections {
    run_line: Option<String>,
    leading_lines: Vec<String>,
    given_lines: Vec<String>,
    formula_lines: Vec<String>,
    simplify_lines: Vec<String>,
    equation_lines: Vec<String>,
    find_lines: Vec<String>,
    solve_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Leading,
    Given,
    Formula,
    Simplify,
    Equation,
    Find,
    Solve,
}

impl MathSections {
    pub fn from_source(source: &str) -> Result<Self, Numora> {
        let mut sections = MathSections::default();
        let mut current_section = Section::Leading;

        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("@run") {
                if sections.run_line.is_none() {
                    sections.run_line = Some(line.to_string());
                }
                continue;
            }

            match trimmed {
                "given:" => {
                    current_section = Section::Given;
                    continue;
                }
                "formula:" => {
                    current_section = Section::Formula;
                    continue;
                }
                "simplify:" => {
                    current_section = Section::Simplify;
                    continue;
                }
                "equation:" => {
                    current_section = Section::Equation;
                    continue;
                }
                "find:" => {
                    current_section = Section::Find;
                    continue;
                }
                "solve:" => {
                    current_section = Section::Solve;
                    continue;
                }
                _ => {}
            }

            match current_section {
                Section::Leading => sections.leading_lines.push(line.to_string()),
                Section::Given => sections.given_lines.push(line.to_string()),
                Section::Formula => sections.formula_lines.push(line.to_string()),
                Section::Simplify => sections.simplify_lines.push(line.to_string()),
                Section::Equation => sections.equation_lines.push(line.to_string()),
                Section::Find => sections.find_lines.push(line.to_string()),
                Section::Solve => sections.solve_lines.push(line.to_string()),
            }
        }

        Ok(sections)
    }

    pub fn merge(&mut self, other: MathSections) {
        if self.run_line.is_none() {
            self.run_line = other.run_line;
        }

        self.leading_lines.extend(other.leading_lines);
        self.given_lines.extend(other.given_lines);
        self.formula_lines.extend(other.formula_lines);
        self.simplify_lines.extend(other.simplify_lines);
        self.equation_lines.extend(other.equation_lines);
        self.find_lines.extend(other.find_lines);
        self.solve_lines.extend(other.solve_lines);
    }

    pub fn to_source(&self) -> String {
        let mut output = String::new();

        if let Some(run_line) = &self.run_line {
            output.push_str(run_line.trim_end());
            output.push_str("\n\n");
        }

        Self::push_section(&mut output, "given:", &self.given_lines);
        Self::push_section(&mut output, "formula:", &self.formula_lines);
        Self::push_section(&mut output, "simplify:", &self.simplify_lines);
        Self::push_section(&mut output, "equation:", &self.equation_lines);
        Self::push_section(&mut output, "find:", &self.find_lines);
        Self::push_section(&mut output, "solve:", &self.solve_lines);

        output.trim().to_string()
    }

    fn push_section(output: &mut String, title: &str, lines: &[String]) {
        let cleaned_lines: Vec<&String> = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .collect();

        if cleaned_lines.is_empty() {
            return;
        }

        output.push_str(title);
        output.push('\n');

        for line in cleaned_lines {
            output.push_str(line.trim_end());
            output.push('\n');
        }

        output.push('\n');
    }
}

pub fn merge_sources(sources: &[String]) -> Result<String, Numora> {
    let mut merged = MathSections::default();

    for source in sources {
        let sections = MathSections::from_source(source)?;
        merged.merge(sections);
    }

    Ok(merged.to_source())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_duplicate_given_sections() {
        let a = r#"
@run calculator

given:
    x = 2
"#;

        let b = r#"
given:
    y = 3

formula:
    result = x + y

find:
    result
"#;

        let merged = merge_sources(&[a.to_string(), b.to_string()]).unwrap();

        assert!(merged.contains("@run calculator"));
        assert!(merged.contains("given:"));
        assert!(merged.contains("x = 2"));
        assert!(merged.contains("y = 3"));
        assert_eq!(merged.matches("given:").count(), 1);
    }

    #[test]
    fn merges_formula_sections() {
        let a = r#"
@run calculator

formula:
    a = 1 + 2
"#;

        let b = r#"
formula:
    b = a + 3

find:
    b
"#;

        let merged = merge_sources(&[a.to_string(), b.to_string()]).unwrap();

        assert_eq!(merged.matches("formula:").count(), 1);
        assert!(merged.contains("a = 1 + 2"));
        assert!(merged.contains("b = a + 3"));
    }

    #[test]
    fn merges_simplify_sections() {
        let a = r#"
@run algebra steps

simplify:
    a = x + 0
"#;

        let b = r#"
simplify:
    b = 1 * y

find:
    a
    b
"#;

        let merged = merge_sources(&[a.to_string(), b.to_string()]).unwrap();

        assert_eq!(merged.matches("simplify:").count(), 1);
        assert!(merged.contains("a = x + 0"));
        assert!(merged.contains("b = 1 * y"));
    }

    #[test]
    fn keeps_first_run_line() {
        let a = r#"
@run algebra steps

given:
    x = 2
"#;

        let b = r#"
@run calculator

given:
    y = 3
"#;

        let merged = merge_sources(&[a.to_string(), b.to_string()]).unwrap();

        assert!(merged.contains("@run algebra steps"));
        assert!(!merged.contains("@run calculator"));
    }
}
