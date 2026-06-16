use crate::error::Numora;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct IncludeResolver;

impl IncludeResolver {
    pub fn expand_source(source: &str) -> Result<String, Numora> {
        let base_dir = std::env::current_dir().map_err(|error| {
            Numora::EvaluationError(format!("Failed to read current directory: {}", error))
        })?;

        Self::expand_source_with_base(source, &base_dir)
    }

    pub fn expand_source_with_base(source: &str, base_dir: &Path) -> Result<String, Numora> {
        let mut visited = HashSet::new();
        Self::expand_recursive(source, base_dir, &mut visited)
    }

    fn expand_recursive(
        source: &str,
        base_dir: &Path,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<String, Numora> {
        let mut expanded = String::new();

        for line in source.lines() {
            let trimmed = line.trim();

            if let Some(include_path) = Self::parse_include(trimmed)? {
                let full_path = base_dir.join(&include_path);
                let canonical_path = full_path.canonicalize().map_err(|error| {
                    Numora::EvaluationError(format!(
                        "Failed to resolve include '{}': {}",
                        include_path.display(),
                        error
                    ))
                })?;

                if visited.contains(&canonical_path) {
                    return Err(Numora::EvaluationError(format!(
                        "Circular include detected: {}",
                        canonical_path.display()
                    )));
                }

                visited.insert(canonical_path.clone());

                let included_source = fs::read_to_string(&canonical_path).map_err(|error| {
                    Numora::EvaluationError(format!(
                        "Failed to read include '{}': {}",
                        canonical_path.display(),
                        error
                    ))
                })?;

                let included_base_dir = canonical_path.parent().unwrap_or(base_dir);

                let included_expanded =
                    Self::expand_recursive(&included_source, included_base_dir, visited)?;

                expanded.push_str(&included_expanded);
                expanded.push('\n');

                visited.remove(&canonical_path);
            } else {
                expanded.push_str(line);
                expanded.push('\n');
            }
        }

        Ok(expanded)
    }

    fn parse_include(line: &str) -> Result<Option<PathBuf>, Numora> {
        if !line.starts_with("@include") {
            return Ok(None);
        }

        let rest = line.trim_start_matches("@include").trim();

        if !rest.starts_with('"') || !rest.ends_with('"') {
            return Err(Numora::ParserError(format!(
                "Invalid include syntax: {}. Expected @include \"path/to/file.mth\"",
                line
            )));
        }

        let path = rest.trim_matches('"').trim();

        if path.is_empty() {
            return Err(Numora::ParserError(
                "Include path cannot be empty".to_string(),
            ));
        }

        Ok(Some(PathBuf::from(path)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_test_dir(name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("numora_include_test_{}", name));

        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        dir
    }

    #[test]
    fn expands_single_include() {
        let dir = temp_test_dir("single");

        fs::write(
            dir.join("common.mth"),
            r#"
given:
    x = 2
"#,
        )
        .unwrap();

        let source = r#"
@run calculator
@include "common.mth"

formula:
    result = x + 3

find:
    result
"#;

        let expanded = IncludeResolver::expand_source_with_base(source, &dir).unwrap();

        assert!(expanded.contains("x = 2"));
        assert!(expanded.contains("result = x + 3"));
    }

    #[test]
    fn expands_nested_include() {
        let dir = temp_test_dir("nested");

        fs::create_dir_all(dir.join("math")).unwrap();

        fs::write(
            dir.join("math").join("vars.mth"),
            r#"
given:
    x = 10
"#,
        )
        .unwrap();

        fs::write(
            dir.join("common.mth"),
            r#"
@include "math/vars.mth"
"#,
        )
        .unwrap();

        let source = r#"
@run calculator
@include "common.mth"

formula:
    result = x + 5

find:
    result
"#;

        let expanded = IncludeResolver::expand_source_with_base(source, &dir).unwrap();

        assert!(expanded.contains("x = 10"));
        assert!(expanded.contains("result = x + 5"));
    }

    #[test]
    fn rejects_bad_include_syntax() {
        let dir = temp_test_dir("bad_syntax");

        let source = r#"
@run calculator
@include common.mth
"#;

        let result = IncludeResolver::expand_source_with_base(source, &dir);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_circular_include() {
        let dir = temp_test_dir("circular");

        fs::write(
            dir.join("a.mth"),
            r#"
@include "b.mth"
"#,
        )
        .unwrap();

        fs::write(
            dir.join("b.mth"),
            r#"
@include "a.mth"
"#,
        )
        .unwrap();

        let source = r#"
@run calculator
@include "a.mth"
"#;

        let result = IncludeResolver::expand_source_with_base(source, &dir);

        assert!(result.is_err());
    }
}
