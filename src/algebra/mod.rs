pub mod expression;
pub mod simplify;
pub mod solve;

pub use expression::{AlgebraExpr, AlgebraOp};
pub use simplify::simplify_expression;
pub use solve::{solve_linear_equation, LinearEquation};
