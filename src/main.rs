use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::Path;

use numora::config::LanguageConfig;
use numora::runtime::Runtime;

fn main() {
    let config = LanguageConfig::default();
    let runtime = Runtime::new(config);

    let args: Vec<String> = env::args().collect();

    match run_app(&runtime, &args) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}

fn run_app(runtime: &Runtime, args: &[String]) -> Result<(), String> {
    if args.len() > 1 {
        return run_from_args(runtime, args);
    }

    if !io::stdin().is_terminal() {
        return run_from_stdin(runtime);
    }

    run_repl(runtime)
}

fn run_from_args(runtime: &Runtime, args: &[String]) -> Result<(), String> {
    if args.len() >= 3 && args[1] == "--file" {
        let path = &args[2];
        return run_file(runtime, path);
    }

    if args.len() >= 2 && args[1] == "--help" {
        print_help();
        return Ok(());
    }

    let expression = args[1..].join(" ");

    match runtime.run(&expression) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(error) => Err(error.to_string()),
    }
}

fn run_file(runtime: &Runtime, path: &str) -> Result<(), String> {
    let path_obj = Path::new(path);

    if path_obj.extension().and_then(|ext| ext.to_str()) != Some("mth") {
        return Err(format!(
            "Invalid file type '{}'. Kara Math files must use .mth extension",
            path
        ));
    }

    let source = fs::read_to_string(path)
        .map_err(|err| format!("Could not read file '{}': {}", path, err))?;

    match runtime.run(&source) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(error) => Err(error.to_string()),
    }
}

fn run_from_stdin(runtime: &Runtime) -> Result<(), String> {
    let mut source = String::new();

    io::stdin()
        .read_to_string(&mut source)
        .map_err(|err| format!("Could not read stdin: {}", err))?;

    match runtime.run(&source) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(error) => Err(error.to_string()),
    }
}

fn run_repl(runtime: &Runtime) -> Result<(), String> {
    println!("Numora Math v0.1");
    println!("Roadmap: Calculator -> Variables -> Steps -> Equations -> Units -> IDE");
    println!();
    println!("Examples:");
    println!("  1 + 2 * 3");
    println!("  sqrt(25)");
    println!("  sumof(1, 2, 3)");
    println!("  PI * 2");
    println!();
    println!("File:");
    println!("  kara_math --file example.mth");
    println!();
    println!("Pipe:");
    println!("  echo \"sqrt(25)\" | NumoraMath");
    println!();
    println!("Type 'exit' to quit.");
    println!();

    loop {
        print!("kara> ");
        io::stdout()
            .flush()
            .map_err(|err| format!("Could not flush stdout: {}", err))?;

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .map_err(|err| format!("Could not read input: {}", err))?;

        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        match runtime.run(input) {
            Ok(output) => println!("{}", output),
            Err(error) => println!("{}", error),
        }
    }

    Ok(())
}

fn print_help() {
    println!("Numora Math");
    println!();
    println!("Usage:");
    println!("  numora");
    println!("  numora \"1 + 2 * 3\"");
    println!("  numora --file example.mth");
    println!("  echo \"sqrt(25)\" | numora");
    println!();
    println!("Supported now:");
    println!("  + - _ * / ^");
    println!("  parentheses");
    println!("  constants: PI, E, e, TAU, PHI");
    println!("  functions: sumof, avgof, minof, maxof");
    println!("  functions: sqrt, abs, round, floor, ceil");
    println!("  functions: sin, cos, tan, ln, log");
}
