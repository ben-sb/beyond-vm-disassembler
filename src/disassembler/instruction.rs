use std::fmt::Display;

use super::operand::Operand;

#[derive(Debug)]
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

#[derive(Debug)]
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

    /// Returns the address of the instruction.
    pub fn address(&self) -> &usize {
        &self.address
    }

    /// Returns the mnemonic of the instruction.
    pub fn mnemonic(&self) -> &Mnemonic {
        &self.mnemonic
    }

    /// Returns the operands of the instruction.
    pub fn operands(&self) -> &Vec<Operand> {
        &self.operands
    }

    /// Returns a string representation of the instruction.
    pub fn to_string(&self) -> String {
        let operand_str = self
            .operands
            .iter()
            .map(|op| op.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "0x{:x}:\t{:<12}{}",
            self.address,
            self.mnemonic.to_string(),
            operand_str
        )
    }
}
