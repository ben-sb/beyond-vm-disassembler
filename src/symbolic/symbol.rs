#[derive(Debug)]
pub enum Symbol {
    UnaryExpressionSymbol(UnaryExpressionSymbol),
    BinaryExpressionSymbol(BinaryExpressionSymbol),
    MemberExpressionSymbol(MemberExpressionSymbol),
}

#[derive(Debug)]
enum UnaryOperator {
    Plus,
    Negate,
    Not,
    BitwiseNot,
}

#[derive(Debug)]
pub struct UnaryExpressionSymbol {
    operator: UnaryOperator,
    argument: Box<Symbol>,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    Xor,
}

#[derive(Debug)]
pub struct BinaryExpressionSymbol {
    operator: BinaryOperator,
    left: Box<Symbol>,
    right: Box<Symbol>,
}

#[derive(Debug)]
pub struct MemberExpressionSymbol {
    object: Box<Symbol>,
    property: Box<Symbol>,
}
