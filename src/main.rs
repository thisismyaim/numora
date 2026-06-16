use numora::config::LanguageConfig;
use numora::runtime::Runtime;
use std::env;
use std::io::{self, IsTerminal, Write};
use std::path::Path;

fn main() {
    let runtime = Runtime::new(LanguageConfig::default());

    let args: Vec<String> = env::args().skip(1).collect();

    if !io::stdin().is_terminal() {
        run_from_stdin(&runtime);
        return;
    }

    if args.is_empty() {
        run_repl(&runtime);
        return;
    }

    if args.len() == 1 {
        let input = &args[0];

        if looks_like_file_path(input) {
            run_file(&runtime, input);
        } else {
            run_expression(&runtime, input);
        }

        return;
    }

    let expression = args.join(" ");
    run_expression(&runtime, &expression);
}

fn run_file(runtime: &Runtime, file_path: &str) {
    match runtime.run_file(file_path) {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("Evaluation Error: {}", error),
    }
}

fn run_expression(runtime: &Runtime, expression: &str) {
    match runtime.run(expression) {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("Evaluation Error: {}", error),
    }
}

fn run_from_stdin(runtime: &Runtime) {
    let source = match io::read_to_string(io::stdin()) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("Input Error: failed to read from stdin: {}", error);
            return;
        }
    };

    if source.trim().is_empty() {
        return;
    }

    match runtime.run(&source) {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("Evaluation Error: {}", error),
    }
}

fn run_repl(runtime: &Runtime) {
    println!("Numora REPL");
    println!("Type an expression or .mth program.");
    println!("Commands: :exit, :quit, :help");
    println!();

    loop {
        print!("numora> ");

        if let Err(error) = io::stdout().flush() {
            eprintln!("Output Error: failed to flush stdout: {}", error);
            return;
        }

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                return;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Input Error: failed to read line: {}", error);
                return;
            }
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            ":exit" | ":quit" => return,
            ":help" => {
                print_help();
                continue;
            }
            _ => {}
        }

        if looks_like_file_path(trimmed) {
            run_file(runtime, trimmed);
        } else {
            run_expression(runtime, trimmed);
        }
    }
}

fn print_help() {
    println!();
    println!("Numora usage:");
    println!();
    println!("  Run direct expression:");
    println!("    numora \"1 + 2 * 3\"");
    println!();
    println!("  Run .mth file:");
    println!("    numora examples/projects/algebra_include_example.mth");
    println!();
    println!("  Pipe source:");
    println!("    cat examples/basic.mth | numora");
    println!();
    println!("REPL commands:");
    println!("  :help   Show help");
    println!("  :exit   Exit REPL");
    println!("  :quit   Exit REPL");
    println!();
}

fn looks_like_file_path(input: &str) -> bool {
    let path = Path::new(input);

    if path.extension().is_some_and(|extension| extension == "mth") {
        return true;
    }

    path.exists() && path.is_file()
}
