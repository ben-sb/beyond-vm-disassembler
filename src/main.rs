use std::fs;

use crate::disassembler::disassembler::Disassembler;

pub mod disassembler;
pub mod symbolic;

fn main() {
    let bytecode = fs::read_to_string("input/bytecode.txt").expect("Failed to read bytecode");
    let algo_bytecode = fs::read_to_string("input/algo.txt").expect("Failed to read algo bytecode");

    let mut disassembler = Disassembler::new(bytecode);
    disassembler.disassemble();
    disassembler.disassemble_snippet("algorithm".to_string(), algo_bytecode);

    let disassembly = disassembler.get_disassembly();
    fs::write("output/disassembly.txt", disassembly.join("\n"))
        .expect("Failed to write disassembly to file");
}
