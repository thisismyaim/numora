# Numora

**Numora** is a Rust-based executable math language and runtime.

It helps users write math in a simple, readable format, attach values to formulas, run calculations, solve equations, work with units, and get useful results from `.mth` files or command-line input.

Numora is not trying to be MATLAB, Mathematica, or Python. Its goal is different:

> **Give values → attach formula → run → get result**

Later, Numora will also support richer step-by-step solving, better unit syntax, JSON/API output, and an IDE called **Numora Studio**.

---

## Current Features

- Basic arithmetic
- Variables
- Built-in constants
- Built-in math functions
- `.mth` math files
- CLI execution
- Pipe/stdin execution
- Step output mode
- Basic numeric equation solving
- Basic unit support
- Rust-first architecture
- No unsafe Rust
- Clean module-based compiler/runtime structure

---

## Supported Arithmetic

```text
1 + 2
10 - 3
10 _ 3
2 * 5
20 / 4
2^3
(1 + 2) * 3
-5 + 10
```

`_` is currently supported as an alias for minus:

```text
10 _ 3
```

same as:

```text
10 - 3
```

---

## Built-in Constants

```text
PI
pi
E
e
TAU
tau
PHI
phi
```

Example:

```bash
cargo run -- "PI * 2"
```

Output:

```text
result: 6.283185307179586
```

---

## Built-in Functions

```text
sumof(1, 2, 3)
avgof(10, 20, 30)
minof(10, 2, 30)
maxof(10, 2, 30)

sqrt(25)
abs(-10)
round(3.6)
floor(3.6)
ceil(3.2)

sin(PI / 2)
cos(0)
tan(PI / 4)

ln(e)
log(100)
```

Example:

```bash
cargo run -- "sqrt(25)"
```

Output:

```text
result: 5
```

---

## `.mth` File Format

Numora uses `.mth` files for math programs.

Basic structure:

```toon
@run calculator

given:
    n = 7

formula:
    f = (n + 1) / 4

find:
    f
```

Run:

```bash
cargo run -- --file examples/basic.mth
```

Output:

```text
result: f = 2
```

---

## Calculator Mode

Example:

```toon
@run calculator

given:
    radius = 5

formula:
    area = PI * radius^2

find:
    area
```

Output:

```text
result: area = 78.53981633974483
```

---

## Steps Mode

Example:

```toon
@run steps

given:
    n = 7

formula:
    f = (n + 1) / 4

find:
    f
```

Output:

```text
result: f = 2

steps:
    f = ((n + 1) / 4)
    f = ((7 + 1) / 4)
    f = (8 / 4)
    f = 2
```

Steps mode is still early and will improve over time.

---

## Solve Mode

Example:

```toon
@run solve

given:
    a = 3
    b = 4

equation:
    a^2 + b^2 = c^2

solve:
    c
```

Run:

```bash
cargo run -- --file examples/solve_triangle.mth
```

Output:

```text
result: c = 5
```

The current equation solver is numeric. It searches for a value that makes both sides of the equation equal.

Example:

```toon
@run solve

equation:
    x + 3 = 10

solve:
    x
```

Output:

```text
result: x = 7
```

---

## Unit Support

Numora has basic unit support.

Currently supported units:

```text
m
cm
km
s
kg
g
```

Example:

```toon
@run calculator

given:
    length = 5 m
    width = 4 m

formula:
    area = length * width

find:
    area
```

Output:

```text
result: area = 20 m^2
```

Speed example:

```toon
@run calculator

given:
    distance = 100 m
    time = 20 s

formula:
    speed = distance / time

find:
    speed
```

Output:

```text
result: speed = 5 m/s
```

Invalid unit example:

```toon
@run calculator

given:
    length = 5 m
    time = 2 s

formula:
    bad = length + time

find:
    bad
```

Output:

```text
Evaluation Error: Cannot add values with different units: 'm' and 's'
```

---

## Command-Line Usage

Run a direct expression:

```bash
cargo run -- "1 + 2 * 3"
```

Output:

```text
result: 7
```

Run a `.mth` file:

```bash
cargo run -- --file examples/basic.mth
```

Pipe input:

```bash
echo "sqrt(25)" | cargo run
```

Build release:

```bash
cargo build --release
```

Run release binary:

```bash
./target/release/numora "sumof(1, 2, 3)"
```

Pipe into release binary:

```bash
echo "sqrt(25)" | ./target/release/numora
```

---

## Example Files

Recommended examples folder:

```text
examples/
├── basic.mth
├── steps.mth
├── solve_x.mth
├── solve_triangle.mth
├── units_area.mth
├── units_speed.mth
└── units_bad_add.mth
```

---

## Project Structure

```text
src/
├── ast.rs
├── builtins.rs
├── config.rs
├── environment.rs
├── error.rs
├── evaluator.rs
├── format.rs
├── lexer.rs
├── lib.rs
├── main.rs
├── parser.rs
├── program.rs
├── runtime/
│   └── mod.rs
├── solver.rs
├── token.rs
├── tracer.rs
└── value.rs
```

### Main Modules

| File | Purpose |
|---|---|
| `lexer.rs` | Converts source text into tokens |
| `token.rs` | Defines token types |
| `parser.rs` | Converts tokens into AST |
| `ast.rs` | Defines expression structure |
| `evaluator.rs` | Evaluates AST into values |
| `value.rs` | Handles numbers and units |
| `environment.rs` | Stores variables |
| `builtins.rs` | Built-in constants and functions |
| `solver.rs` | Numeric equation solving |
| `tracer.rs` | Step-by-step output |
| `program.rs` | Parses and runs `.mth` programs |
| `runtime/` | Main runtime interface |
| `main.rs` | CLI entry point |
| `lib.rs` | Library exports for tests and future API |

---

## Roadmap

Current roadmap:

```text
Calculator              ✅
Built-in constants      ✅
Built-in functions      ✅
.mth files              ✅
Pipe support            ✅
Variables               ✅
Steps                   ✅
Equations               ✅ basic numeric solver
Units                   ✅ basic
Quality pass            in progress
IDE                     later
```

Future planned work:

- Better line-number errors
- JSON output mode
- More unit syntax like `m^2`, `m/s`
- More unit conversions
- Multiple equation solutions
- Better symbolic solving
- Improved step-by-step explanations
- Complex number support with `i`
- Web/API runtime
- Numora Studio IDE

---

## Development

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

Run direct expression:

```bash
cargo run -- "1 + 2 * 3"
```

Run file:

```bash
cargo run -- --file examples/basic.mth
```

Run tests:

```bash
cargo test
```

Build optimized binary:

```bash
cargo build --release
```

---

## Contributing

Contributions are welcome.

Numora is still early, so the best contributions are small, focused, and easy to review.

Good first contribution areas:

- Add more tests
- Improve error messages
- Improve examples
- Improve documentation
- Add more built-in functions
- Improve unit formatting
- Add line-number errors
- Improve step-by-step tracing
- Add JSON output mode

---

## Contribution Guidelines

### 1. Keep changes focused

Prefer small pull requests.

Good:

```text
Add tests for unit addition
```

Not ideal:

```text
Rewrite parser, add IDE, add JSON, change syntax, and rename all modules
```

### 2. Add tests

If you add or change behavior, add tests.

Run:

```bash
cargo test
```

before opening a pull request.

### 3. Keep code readable

Numora is also a learning project. Code should be clear and easy to understand.

Prefer:

```rust
let result = left_value.add(right_value)?;
```

over clever but hard-to-read code.

### 4. Avoid unsafe Rust

Do not use `unsafe` unless there is a strong reason and it is discussed first.

### 5. Keep user-facing errors friendly

Bad:

```text
Unexpected token
```

Better:

```text
Parser Error: Expected a number, symbol, function, or '(' but found '+'
```

### 6. Respect the roadmap

Current order:

```text
Calculator -> Variables -> Steps -> Equations -> Units -> Quality -> IDE
```

Do not jump directly into IDE features before the runtime is stable.

---

## Example Contribution Workflow

Fork the project, then:

```bash
git clone https://github.com/YOUR_USERNAME/numora.git
cd numora
cargo build
cargo test
```

Create a branch:

```bash
git checkout -b add-more-unit-tests
```

Make changes, then:

```bash
cargo fmt
cargo test
git add .
git commit -m "Add more unit tests"
git push origin add-more-unit-tests
```

Open a pull request.

---

## License

License is not selected yet.

Recommended options:

- MIT for simple open-source use
- Apache-2.0 for stronger patent protection
- MIT OR Apache-2.0 for Rust ecosystem style

Until a license is added, all rights are reserved by default.

---

## Status

Numora is experimental and under active development.

It currently works as a CLI math runtime and `.mth` file executor. The architecture is being prepared for future API and IDE support.

---

## Vision

Numora aims to become a beginner-friendly executable math language.

Instead of forcing users to mentally translate formulas into normal programming code, Numora lets them write math in a structured way:

```toon
given:
    a = 3
    b = 4

equation:
    a^2 + b^2 = c^2

solve:
    c
```

and get:

```text
result: c = 5
```

The long-term vision is:

```text
Values + formulas + units + equations + steps + IDE
```

A math language that explains, computes, and grows with the learner.
