#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),

    Quantity {
        number: f64,
        unit: String,
    },

    Symbol {
        name: String,
    },

    FunctionCall {
        name: String,
        arguments: Vec<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },

    Unary {
        operator: UnaryOperator,
        expression: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negative,
}
