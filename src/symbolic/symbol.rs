use std::fmt::Display;

use crate::disassembler::operand::{Literal, Operand, Variable};

#[derive(Debug)]
pub enum Symbol {
    LiteralSymbol(LiteralSymbol),
    IdentifierSymbol(IdentifierSymbol),
    UnaryExpressionSymbol(UnaryExpressionSymbol),
    BinaryExpressionSymbol(BinaryExpressionSymbol),
    CallExpressionSymbol(CallExpressionSymbol),
}

impl Display for Symbol {
    /// Writes a readable version of the symbol.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::LiteralSymbol(literal) => literal.fmt(f),
            Symbol::IdentifierSymbol(ident) => ident.fmt(f),
            Symbol::UnaryExpressionSymbol(unary) => unary.fmt(f),
            Symbol::BinaryExpressionSymbol(binary) => binary.fmt(f),
            Symbol::CallExpressionSymbol(call) => call.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum LiteralSymbol {
    Zero,
    NegZero,
    Infinity,
    NegInfinity,
    One, // special one, only used for 1/x
}

impl Display for LiteralSymbol {
    /// Writes a readable version of the symbol.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep = match self {
            LiteralSymbol::Zero => "0.0",
            LiteralSymbol::NegZero => "-0.0",
            LiteralSymbol::Infinity => "Infinity",
            LiteralSymbol::NegInfinity => "-Infinity",
            LiteralSymbol::One => "1",
        };
        write!(f, "{}", rep)
    }
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

impl Display for IdentifierSymbol {
    /// Writes a readable version of the symbol.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
}

impl Display for UnaryOperator {
    /// Writes a readable version of the operator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "!")
    }
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

impl Display for UnaryExpressionSymbol {
    /// Writes a readable version of the symbol.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.argument)
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for BinaryOperator {
    /// Writes a readable version of the operator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        };
        write!(f, "{operator}")
    }
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

impl Display for BinaryExpressionSymbol {
    /// Writes a readable version of the symbol.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
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

impl Display for CallExpressionSymbol {
    /// Writes a readable version of the symbol.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.callee)?;
        for (index, arg) in self.arguments.iter().enumerate() {
            let separator = if index < self.arguments.len() - 1 {
                ", "
            } else {
                ""
            };
            write!(f, "{}{}", arg, separator)?;
        }
        write!(f, ")")
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
