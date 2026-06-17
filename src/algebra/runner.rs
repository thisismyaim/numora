use crate::algebra::{explain_simplification, parse_algebra_expression, AlgebraExplanation};
use crate::error::Numora;

#[derive(Debug, Clone, PartialEq, Eq)]
struct SimplifyAssignment {
    name: String,
    expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NamedExplanation {
    name: String,
    explanation: AlgebraExplanation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AlgebraSection {
    None,
    Simplify,
    Find,
}

pub fn source_contains_simplify_section(source: &str) -> bool {
    source.lines().any(|line| line.trim() == "simplify:")
}

pub fn run_algebra_simplify_program(source: &str) -> Result<String, Numora> {
    let wants_steps = source
        .lines()
        .find(|line| line.trim().starts_with("@run"))
        .map(|line| line.split_whitespace().any(|part| part == "steps"))
        .unwrap_or(false);

    let assignments = parse_simplify_assignments(source)?;
    let finds = parse_find_items(source)?;

    if assignments.is_empty() {
        return Err(Numora::EvaluationError(
            "@run algebra simplify needs at least one simplify assignment".to_string(),
        ));
    }

    let mut results = Vec::new();

    for assignment in assignments {
        let parsed = parse_algebra_expression(&assignment.expression)?;
        let explanation = explain_simplification(parsed);

        results.push(NamedExplanation {
            name: assignment.name,
            explanation,
        });
    }

    let selected_results = select_results(results, finds)?;

    Ok(format_simplify_results(&selected_results, wants_steps))
}

fn parse_simplify_assignments(source: &str) -> Result<Vec<SimplifyAssignment>, Numora> {
    let mut assignments = Vec::new();
    let mut section = AlgebraSection::None;

    for line in source.lines() {
        let trimmed = line.trim();

        if should_skip_line(trimmed) {
            continue;
        }

        match trimmed {
            "simplify:" => {
                section = AlgebraSection::Simplify;
                continue;
            }
            "find:" => {
                section = AlgebraSection::Find;
                continue;
            }
            "given:" | "formula:" | "equation:" | "solve:" => {
                section = AlgebraSection::None;
                continue;
            }
            _ => {}
        }

        if section == AlgebraSection::Simplify {
            let (name, expression) = trimmed.split_once('=').ok_or_else(|| {
                Numora::ParserError(format!(
                    "Invalid simplify assignment '{}'. Expected name = expression",
                    trimmed
                ))
            })?;

            let name = name.trim();

            if name.is_empty() {
                return Err(Numora::ParserError(
                    "Simplify assignment name cannot be empty".to_string(),
                ));
            }

            let expression = expression.trim();

            if expression.is_empty() {
                return Err(Numora::ParserError(format!(
                    "Simplify assignment '{}' has empty expression",
                    name
                )));
            }

            assignments.push(SimplifyAssignment {
                name: name.to_string(),
                expression: expression.to_string(),
            });
        }
    }

    Ok(assignments)
}

fn parse_find_items(source: &str) -> Result<Vec<String>, Numora> {
    let mut finds = Vec::new();
    let mut section = AlgebraSection::None;

    for line in source.lines() {
        let trimmed = line.trim();

        if should_skip_line(trimmed) {
            continue;
        }

        match trimmed {
            "find:" => {
                section = AlgebraSection::Find;
                continue;
            }
            "simplify:" | "given:" | "formula:" | "equation:" | "solve:" => {
                section = AlgebraSection::None;
                continue;
            }
            _ => {}
        }

        if section == AlgebraSection::Find {
            for item in trimmed.split(',') {
                let item = item.trim();

                if !item.is_empty() {
                    finds.push(item.to_string());
                }
            }
        }
    }

    Ok(finds)
}

fn select_results(
    results: Vec<NamedExplanation>,
    finds: Vec<String>,
) -> Result<Vec<NamedExplanation>, Numora> {
    if finds.is_empty() {
        return Ok(results);
    }

    let mut selected = Vec::new();

    for find in finds {
        if let Some(result) = results.iter().find(|result| result.name == find) {
            selected.push(result.clone());
        } else {
            return Err(Numora::EvaluationError(format!(
                "Could not find simplify result '{}'",
                find
            )));
        }
    }

    Ok(selected)
}

fn format_simplify_results(results: &[NamedExplanation], wants_steps: bool) -> String {
    let mut output = String::new();

    for result in results {
        output.push_str(&format!(
            "result: {} = {}\n",
            result.name, result.explanation.simplified
        ));
    }

    if wants_steps {
        output.push_str("\nsteps:\n");

        for result in results {
            output.push_str(&format!(
                "    original: {} = {}\n",
                result.name, result.explanation.original
            ));
            output.push_str(&format!(
                "    simplified: {} = {}\n",
                result.name, result.explanation.simplified
            ));

            for step in &result.explanation.steps {
                output.push_str(&format!("    rule: {}\n", step.rule));
                output.push_str(&format!("    explanation: {}\n", step.explanation));
            }
        }
    }

    output.trim_end().to_string()
}

fn should_skip_line(line: &str) -> bool {
    line.is_empty() || line.starts_with('#') || line.starts_with("@run")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_simplify_section() {
        let source = r#"
@run algebra

simplify:
    result = x + 0
"#;

        assert!(source_contains_simplify_section(source));
    }

    #[test]
    fn runs_simple_simplify_program() {
        let source = r#"
@run algebra

simplify:
    result = x + 0

find:
    result
"#;

        let output = run_algebra_simplify_program(source).unwrap();

        assert!(output.contains("result: result = x"));
    }

    #[test]
    fn runs_simplify_program_with_steps() {
        let source = r#"
@run algebra steps

simplify:
    result = x + 0

find:
    result
"#;

        let output = run_algebra_simplify_program(source).unwrap();

        assert!(output.contains("result: result = x"));
        assert!(output.contains("steps:"));
        assert!(output.contains("original:"));
        assert!(output.contains("simplified:"));
        assert!(output.contains("rule:"));
        assert!(output.contains("explanation:"));
    }

    #[test]
    fn simplifies_numeric_expression() {
        let source = r#"
@run algebra

simplify:
    result = 2 + 3 * 4

find:
    result
"#;

        let output = run_algebra_simplify_program(source).unwrap();

        assert!(output.contains("result: result = 14"));
    }

    #[test]
    fn returns_all_results_when_find_is_missing() {
        let source = r#"
@run algebra

simplify:
    a = x + 0
    b = 1 * y
"#;

        let output = run_algebra_simplify_program(source).unwrap();

        assert!(output.contains("result: a = x"));
        assert!(output.contains("result: b = y"));
    }

    #[test]
    fn rejects_missing_find_result() {
        let source = r#"
@run algebra

simplify:
    result = x + 0

find:
    missing
"#;

        let result = run_algebra_simplify_program(source);

        assert!(result.is_err());
    }
}
