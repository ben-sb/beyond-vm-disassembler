use std::fmt::Display;

use super::operand::Operand;

pub enum Mnemonic {
    PUSH,
    POP,
    RET,
    CALL,
    ADD,
    SUB,
    MUL,
    DIV,
    MIN,
    MAX,
    FRAC,
    NEG,
}

impl Display for Mnemonic {
    /// Formats the mnemonic as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Mnemonic::PUSH => "push",
            Mnemonic::POP => "pop",
            Mnemonic::RET => "ret",
            Mnemonic::CALL => "call",
            Mnemonic::ADD => "add",
            Mnemonic::SUB => "sub",
            Mnemonic::MUL => "mul",
            Mnemonic::DIV => "div",
            Mnemonic::MIN => "min",
            Mnemonic::MAX => "max",
            Mnemonic::FRAC => "frac",
            Mnemonic::NEG => "neg",
        };
        write!(f, "{}", repr)
    }
}

pub struct Instruction {
    address: usize,
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
}

impl Instruction {
    /// Creates a new instruction.
    pub fn new(address: usize, mnemonic: Mnemonic, operands: Vec<Operand>) -> Instruction {
        Instruction {
            address,
            mnemonic,
            operands,
        }
    }
}
