use crate::disassembler::operand::{Literal, Operand, Variable};

#[derive(Debug)]
pub enum Symbol {
    LiteralSymbol(LiteralSymbol),
    IdentifierSymbol(IdentifierSymbol),
    UnaryExpressionSymbol(UnaryExpressionSymbol),
    BinaryExpressionSymbol(BinaryExpressionSymbol),
    CallExpressionSymbol(CallExpressionSymbol),
}

#[derive(Debug)]
pub enum LiteralSymbol {
    Zero,
    NegZero,
    Infinity,
    NegInfinity,
    One, // special one, only used for 1/x
}

#[derive(Debug)]
pub struct IdentifierSymbol {
    name: String,
}

impl IdentifierSymbol {
    /// Creates a new identifier symbol.
    pub fn new(name: String) -> IdentifierSymbol {
        IdentifierSymbol { name }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug)]
pub struct UnaryExpressionSymbol {
    operator: UnaryOperator,
    argument: Box<Symbol>,
}

impl UnaryExpressionSymbol {
    /// Creates a new unary expression symbol.
    pub fn new(operator: UnaryOperator, argument: Box<Symbol>) -> UnaryExpressionSymbol {
        UnaryExpressionSymbol { operator, argument }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct BinaryExpressionSymbol {
    operator: BinaryOperator,
    left: Box<Symbol>,
    right: Box<Symbol>,
}

impl BinaryExpressionSymbol {
    /// Creates a new binary expression symbol.
    pub fn new(
        operator: BinaryOperator,
        left: Box<Symbol>,
        right: Box<Symbol>,
    ) -> BinaryExpressionSymbol {
        BinaryExpressionSymbol {
            operator,
            left,
            right,
        }
    }
}

#[derive(Debug)]
pub struct CallExpressionSymbol {
    callee: Box<Symbol>,
    arguments: Vec<Symbol>,
}

impl CallExpressionSymbol {
    /// Creates a new call expression symbol.
    pub fn new(callee: Box<Symbol>, arguments: Vec<Symbol>) -> CallExpressionSymbol {
        CallExpressionSymbol { callee, arguments }
    }
}

impl From<&Operand> for Symbol {
    /// Creates a symbol from an operand.
    fn from(value: &Operand) -> Self {
        match value {
            Operand::Literal(literal) => match literal {
                Literal::ZERO => Symbol::LiteralSymbol(LiteralSymbol::Zero),
                Literal::INFINITY => Symbol::LiteralSymbol(LiteralSymbol::Infinity),
            },
            Operand::GlobalVariable(var) => {
                Symbol::IdentifierSymbol(IdentifierSymbol::new(var.name().to_string()))
            }
            Operand::Parameter(param) => {
                Symbol::IdentifierSymbol(IdentifierSymbol::new(param.name().to_string()))
            }
            Operand::FunctionReference(func) => {
                Symbol::IdentifierSymbol(IdentifierSymbol::new(func.name().to_string()))
            }
        }
    }
}
