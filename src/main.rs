use std::fs;

use crate::disassembler::{disassembler::Disassembler, instruction::Instruction};

pub mod disassembler;

fn main() {
    let bytecode = fs::read_to_string("input/bytecode.txt").expect("Failed to read bytecode");
    let algo_bytecode = fs::read_to_string("input/algo.txt").expect("Failed to read algo bytecode");

    let mut disassembler = Disassembler::new(bytecode);
    disassembler.disassemble();
    disassembler.disassemble_snippet("algorithm".to_string(), algo_bytecode);
}
