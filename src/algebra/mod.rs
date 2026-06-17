pub mod explain;
pub mod expression;
pub mod parser;
pub mod runner;
pub mod simplify;
pub mod solve;

pub use explain::{explain_simplification, AlgebraExplanation, AlgebraExplanationStep};
pub use expression::{AlgebraExpr, AlgebraOp};
pub use parser::parse_algebra_expression;
pub use runner::{run_algebra_simplify_program, source_contains_simplify_section};
pub use simplify::simplify_expression;
pub use solve::{solve_linear_equation, LinearEquation};
