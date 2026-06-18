use crate::algebra::explain::{explain_simplification, AlgebraExplanation};
use crate::algebra::parser::parse_algebra_expression;
use crate::algebra::simplify_expression;
use crate::error::Numora;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraSimplifyResult {
    name: String,
    simplified: String,
    explanation: AlgebraExplanation,
}

pub fn source_contains_simplify_section(source: &str) -> bool {
    source
        .lines()
        .any(|line| line.trim().eq_ignore_ascii_case("simplify:"))
}

pub fn run_algebra_simplify_program(source: &str) -> Result<String, Numora> {
    let show_steps = source_uses_steps_mode(source);

    run_algebra_simplify_program_with_steps(source, show_steps)
}

pub fn run_algebra_simplify_program_with_steps(
    source: &str,
    show_steps: bool,
) -> Result<String, Numora> {
    let simplify_lines = collect_section_lines(source, "simplify");
    let find_lines = collect_section_lines(source, "find");

    if simplify_lines.is_empty() {
        return Err(algebra_error("Missing simplify section."));
    }

    let mut results = Vec::new();

    for line in simplify_lines {
        let Some((name, expression_source)) = parse_assignment(&line) else {
            return Err(algebra_error(format!(
                "Invalid simplify assignment: {}",
                line
            )));
        };

        let parsed = parse_algebra_expression(expression_source.trim()).map_err(|err| {
            algebra_error(format!(
                "Failed to parse algebra expression '{}': {}",
                expression_source, err
            ))
        })?;

        let simplified = simplify_expression(parsed.clone());
        let explanation = explain_simplification(parsed);

        results.push(AlgebraSimplifyResult {
            name: name.trim().to_string(),
            simplified: simplified.to_string(),
            explanation,
        });
    }

    let requested_names = parse_find_names(&find_lines);

    let selected_results = if requested_names.is_empty() {
        results
    } else {
        let mut selected = Vec::new();

        for requested_name in requested_names {
            let Some(result) = results
                .iter()
                .find(|result| result.name == requested_name)
                .cloned()
            else {
                return Err(algebra_error(format!(
                    "Missing simplify result: {}",
                    requested_name
                )));
            };

            selected.push(result);
        }

        selected
    };

    let mut output = String::new();

    for result in selected_results {
        output.push_str(&format!(
            "result: {} = {}\n",
            result.name, result.simplified
        ));

        output.push_str(&format!(
            "latex: {} = {}\n",
            result.name, result.explanation.latex_simplified
        ));

        if show_steps {
            output.push_str("steps:\n");

            for step in result.explanation.steps {
                output.push_str(&format!("rule: {}\n", step.rule));
                output.push_str(&format!("before: {}\n", step.before));
                output.push_str(&format!("after: {}\n", step.after));
                output.push_str(&format!("latex_before: {}\n", step.latex_before));
                output.push_str(&format!("latex_after: {}\n", step.latex_after));
                output.push_str(&format!("explanation: {}\n", step.explanation));
            }
        }
    }

    Ok(output.trim_end().to_string())
}

fn source_uses_steps_mode(source: &str) -> bool {
    source
        .lines()
        .find(|line| line.trim_start().starts_with("@run"))
        .map(|line| line.split_whitespace().any(|mode| mode == "steps"))
        .unwrap_or(false)
}

fn collect_section_lines(source: &str, section_name: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut inside_target_section = false;

    for raw_line in source.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') || line.starts_with("@run") {
            continue;
        }

        if is_section_header(line) {
            inside_target_section = line.eq_ignore_ascii_case(&format!("{}:", section_name));
            continue;
        }

        if inside_target_section {
            lines.push(line.to_string());
        }
    }

    lines
}

fn is_section_header(line: &str) -> bool {
    matches!(
        line,
        "given:" | "formula:" | "simplify:" | "equation:" | "find:" | "solve:"
    )
}

fn parse_assignment(line: &str) -> Option<(&str, &str)> {
    let (name, expression) = line.split_once('=')?;

    let name = name.trim();
    let expression = expression.trim();

    if name.is_empty() || expression.is_empty() {
        return None;
    }

    Some((name, expression))
}

fn parse_find_names(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn algebra_error(message: impl Into<String>) -> Numora {
    Numora::EvaluationError(message.into())
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

find:
    result
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
        assert!(output.contains("latex: result = x"));
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
        assert!(output.contains("latex: result = x"));
        assert!(output.contains("steps:"));
        assert!(output.contains("rule: Additive identity"));
        assert!(output.contains("latex_before: x + 0"));
        assert!(output.contains("latex_after: x"));
    }

    #[test]
    fn explicit_steps_helper_can_show_steps() {
        let source = r#"
@run algebra

simplify:
    result = x + 0

find:
    result
"#;

        let output = run_algebra_simplify_program_with_steps(source, true).unwrap();

        assert!(output.contains("steps:"));
        assert!(output.contains("latex_before: x + 0"));
        assert!(output.contains("latex_after: x"));
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
        assert!(output.contains("latex: result = 14"));
    }

    #[test]
    fn returns_all_results_when_find_is_missing() {
        let source = r#"
@run algebra

simplify:
    first = x + 0
    second = 1 * y
"#;

        let output = run_algebra_simplify_program(source).unwrap();

        assert!(output.contains("result: first = x"));
        assert!(output.contains("result: second = y"));
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

        let error = run_algebra_simplify_program(source).unwrap_err();

        assert!(error
            .to_string()
            .contains("Missing simplify result: missing"));
    }
}
