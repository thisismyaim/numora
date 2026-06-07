use crate::environment::Environment;
use crate::error::Numora;
use crate::evaluator::evaluate;
use crate::format::format_number;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::solver::{parse_equation, solve_equation, Equation};
use crate::tracer::trace_assignment;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct MathProgram {
    pub run_mode: RunMode,
    pub assignments: Vec<Assignment>,
    pub formula: Option<Assignment>,
    pub equation: Option<Equation>,
    pub find: Option<String>,
    pub solve: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Calculator,
    Steps,
    Solve,
}

pub fn detect_run_mode(source: &str) -> RunMode {
    if source.contains("@run steps") {
        RunMode::Steps
    } else if source.contains("@run solve") {
        RunMode::Solve
    } else {
        RunMode::Calculator
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub expression: String,
}

pub fn run_math_program(source: &str) -> Result<String, Numora> {
    let program = parse_math_program(source)?;

    match program.run_mode {
        RunMode::Calculator => run_calculator_program(&program),
        RunMode::Steps => run_steps_program(&program),
        RunMode::Solve => run_solve_program(&program),
    }
}

fn run_calculator_program(program: &MathProgram) -> Result<String, Numora> {
    let mut env = Environment::new();

    for assignment in &program.assignments {
        let value = evaluate_expression(&assignment.expression, &env)?;
        env.set(assignment.name.clone(), value);
    }

    if let Some(formula) = &program.formula {
        let value = evaluate_expression(&formula.expression, &env)?;
        env.set(formula.name.clone(), value);
    }

    build_result(program, &env)
}

fn run_steps_program(program: &MathProgram) -> Result<String, Numora> {
    let mut env = Environment::new();

    for assignment in &program.assignments {
        let value = evaluate_expression(&assignment.expression, &env)?;
        env.set(assignment.name.clone(), value);
    }

    let Some(formula) = &program.formula else {
        return Err(Numora::EvaluationError(
            "@run steps needs a formula section".to_string(),
        ));
    };

    let ast = parse_expression_to_ast(&formula.expression)?;
    let trace = trace_assignment(&formula.name, &ast, &env)?;

    env.set(formula.name.clone(), trace.value);

    let mut output = build_result(program, &env)?;

    output.push_str("\n\nsteps:");

    for step in trace.steps {
        output.push_str("\n    ");
        output.push_str(&step);
    }

    Ok(output)
}

fn run_solve_program(program: &MathProgram) -> Result<String, Numora> {
    let mut env = Environment::new();

    for assignment in &program.assignments {
        let value = evaluate_expression(&assignment.expression, &env)?;
        env.set(assignment.name.clone(), value);
    }

    let Some(equation) = &program.equation else {
        return Err(Numora::EvaluationError(
            "@run solve needs an equation section".to_string(),
        ));
    };

    let Some(solve_variable) = &program.solve else {
        return Err(Numora::EvaluationError(
            "@run solve needs a solve section".to_string(),
        ));
    };

    let result = solve_equation(equation, solve_variable, &env)?;

    Ok(format!(
        "result: {} = {}",
        result.variable,
        format_number(result.value)
    ))
}

fn build_result(program: &MathProgram, env: &Environment) -> Result<String, Numora> {
    if let Some(find_name) = &program.find {
        if let Some(value) = env.get(find_name) {
            return Ok(format!("result: {} = {}", find_name, value.format()));
        }

        return Err(Numora::EvaluationError(format!(
            "Cannot find '{}'. It was not defined",
            find_name
        )));
    }

    if let Some(formula) = &program.formula {
        if let Some(value) = env.get(&formula.name) {
            return Ok(format!("result: {} = {}", formula.name, value.format()));
        }
    }

    Err(Numora::EvaluationError(
        "Program has no formula or find target".to_string(),
    ))
}

pub fn evaluate_expression(source: &str, env: &Environment) -> Result<Value, Numora> {
    let ast = parse_expression_to_ast(source)?;
    evaluate(&ast, env)
}

fn parse_expression_to_ast(source: &str) -> Result<crate::ast::Expr, Numora> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn parse_math_program(source: &str) -> Result<MathProgram, Numora> {
    let mut program = MathProgram {
        run_mode: RunMode::Calculator,
        assignments: Vec::new(),
        formula: None,
        equation: None,
        find: None,
        solve: None,
    };

    let mut current_section = Section::None;

    for raw_line in source.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        match line {
            "@run calculator" => {
                program.run_mode = RunMode::Calculator;
                continue;
            }

            "@run steps" => {
                program.run_mode = RunMode::Steps;
                continue;
            }

            "@run solve" => {
                program.run_mode = RunMode::Solve;
                continue;
            }

            "given:" => {
                current_section = Section::Given;
                continue;
            }

            "formula:" => {
                current_section = Section::Formula;
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
            Section::Given => {
                let assignment = parse_assignment(line)?;
                program.assignments.push(assignment);
            }

            Section::Formula => {
                let assignment = parse_assignment(line)?;
                program.formula = Some(assignment);
            }

            Section::Equation => {
                let equation = parse_equation(line)?;
                program.equation = Some(equation);
            }

            Section::Find => {
                validate_identifier(line)?;
                program.find = Some(line.to_string());
            }

            Section::Solve => {
                validate_identifier(line)?;
                program.solve = Some(line.to_string());
            }

            Section::None => {
                if let Some(value) = line.strip_prefix("input:") {
                    let expression = value.trim();

                    program.formula = Some(Assignment {
                        name: "answer".to_string(),
                        expression: expression.to_string(),
                    });

                    program.find = Some("answer".to_string());
                } else {
                    return Err(Numora::ParserError(format!(
                        "Line outside a section: '{}'. Expected given:, formula:, equation:, find:, or solve:",
                        line
                    )));
                }
            }
        }
    }

    Ok(program)
}

fn parse_assignment(line: &str) -> Result<Assignment, Numora> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();

    if parts.len() != 2 {
        return Err(Numora::ParserError(format!(
            "Expected assignment like: n = 7, but found '{}'",
            line
        )));
    }

    let name = parts[0].trim();
    let expression = parts[1].trim();

    if name.is_empty() {
        return Err(Numora::ParserError(
            "Assignment name cannot be empty".to_string(),
        ));
    }

    if expression.is_empty() {
        return Err(Numora::ParserError(format!(
            "Assignment '{}' has no expression after '='",
            name
        )));
    }

    validate_identifier(name)?;

    Ok(Assignment {
        name: name.to_string(),
        expression: expression.to_string(),
    })
}

fn validate_identifier(name: &str) -> Result<(), Numora> {
    let mut chars = name.chars();

    let Some(first) = chars.next() else {
        return Err(Numora::ParserError(
            "Identifier cannot be empty".to_string(),
        ));
    };

    if !first.is_ascii_alphabetic() {
        return Err(Numora::ParserError(format!(
            "Identifier '{}' must start with a letter",
            name
        )));
    }

    for ch in chars {
        if !ch.is_ascii_alphanumeric() && ch != '_' {
            return Err(Numora::ParserError(format!(
                "Identifier '{}' contains invalid character '{}'",
                name, ch
            )));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Section {
    None,
    Given,
    Formula,
    Equation,
    Find,
    Solve,
}
