use std::{cmp::min, collections::HashSet};

use super::{
    function::Function,
    instruction::{Instruction, Mnemonic},
    operand::{GlobalVariable, Literal, Operand, Parameter},
};
use lazy_static::lazy_static;

lazy_static! {
    static ref RESERVED_ALPHANUMERIC_OPCODES: HashSet<char> =
        vec!['m', 'M', '0', '1'].into_iter().collect();
}

pub struct Disassembler {
    bytecode: Vec<char>,
    base_address: usize,
    pos: usize,
    functions: Vec<Function>,
    current_function_index: Option<usize>,
    num_functions: usize,
    delimiter: char,
}

impl Disassembler {
    /// Creates a new disassembler.
    pub fn new(bytecode: String) -> Disassembler {
        Disassembler {
            bytecode: bytecode.chars().collect(),
            base_address: 0,
            pos: 0,
            functions: Vec::new(),
            current_function_index: None,
            num_functions: 0,
            delimiter: ';',
        }
    }

    /// Returns the vector of functions.
    pub fn functions(self) -> Vec<Function> {
        self.functions
    }

    /// Reads the next byte from the bytecode and advances the position.
    fn read_byte(&mut self) -> char {
        let byte = self.bytecode[self.pos];
        self.pos += 1;
        return byte;
    }

    /// Reads the next byte from the bytecode, without advancing the position.
    fn peek_byte(&self) -> char {
        return self.bytecode[self.pos];
    }

    /// Advances the position.
    fn advance(&mut self) {
        self.pos += 1;
    }

    /// Adds an instruction to the current function being disassembled.
    fn add_instruction(&mut self, instr: Instruction) {
        match self.current_function_index {
            Some(i) => self.functions[i].add_instruction(instr),
            None => panic!("Tried to add an instruction when there is no current function"),
        };
    }

    /// Returns the disassembly as a vector of disassembly lines.
    pub fn get_disassembly(&self) -> Vec<String> {
        let mut dis: Vec<String> = Vec::new();

        for func in &self.functions {
            dis.append(&mut func.get_disassembly());
            dis.push(String::from("")); // empty line to separate functions
        }

        dis
    }

    /// Disassembles a new snippet of bytecode.
    pub fn disassemble_snippet(&mut self, snippet_name: String, bytecode: String) {
        let func = Function::new(self.base_address + self.pos, snippet_name, 8);
        self.functions.push(func);
        self.current_function_index = Some(self.num_functions);
        self.num_functions += 1;

        self.bytecode = bytecode.chars().collect();
        self.base_address += self.pos; // update base address
        self.pos = 0; // reset pos within bytecode

        self.disassemble();
    }

    /// Disassembles the current bytecode.
    pub fn disassemble(&mut self) {
        let bytecode_length = self.bytecode.len();
        while self.pos < bytecode_length {
            let address = self.base_address + self.pos;
            let opcode = self.read_byte();

            // delimiter means stop disassembling
            if opcode == self.delimiter {
                if let Some(_) = self.current_function_index {
                    let instr = Instruction::new(address, Mnemonic::RET, Vec::new());
                    self.add_instruction(instr);
                    self.current_function_index = None;
                }
                continue;
            }
            // push global variable or parameter
            else if opcode.is_alphanumeric() && !RESERVED_ALPHANUMERIC_OPCODES.contains(&opcode) {
                let next_byte = self.peek_byte();
                if next_byte == '=' {
                    self.advance();
                    // this never appears in the sample
                    panic!("Storing variables as result of subroutines not implemented")
                }

                // letters correspond to global variables
                if opcode.is_alphabetic() {
                    let var = Operand::GlobalVariable(GlobalVariable::new(opcode.to_string()));
                    let instr = Instruction::new(address, Mnemonic::PUSH, vec![var]);
                    self.add_instruction(instr);
                }
                // numbers correspond to parameters
                else {
                    let param_index = (opcode as usize) - 50;
                    let param = Operand::Parameter(Parameter::new(param_index));
                    let instr = Instruction::new(address, Mnemonic::PUSH, vec![param]);
                    self.add_instruction(instr);
                }
            }
            // create function
            else if opcode == ':' {
                let mut name = String::new();
                loop {
                    let next_byte = self.read_byte();
                    if next_byte == ':' {
                        break;
                    } else {
                        name.push(next_byte);
                    }
                }

                let func = Function::new(address, name, 8);
                self.functions.push(func);
                self.current_function_index = Some(self.num_functions);
                self.num_functions += 1;
            }
            // call function
            else if opcode == '^' {
                // Access next 10 chars as a lookahead buffer. Function IDs are <= 3
                // chars so this is more than long enough
                let next_bytecode: String = self.bytecode
                    [self.pos..min(self.pos + 10, bytecode_length)]
                    .iter()
                    .collect();

                // find which function is being called
                let mut callee: Option<&Function> = None;
                for func in &self.functions {
                    if next_bytecode.starts_with(func.id()) {
                        callee = Some(func);
                    }
                }

                if let Some(func) = callee {
                    let operands = vec![Operand::FunctionReference(func.get_reference())];
                    let instr = Instruction::new(address, Mnemonic::CALL, operands);
                    self.add_instruction(instr);
                } else {
                    panic!("Unknown function callee");
                }
            } else {
                match opcode {
                    '+' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::ADD, Vec::new()))
                    }
                    '-' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::SUB, Vec::new()))
                    }
                    '*' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::MUL, Vec::new()))
                    }
                    '/' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::DIV, Vec::new()))
                    }
                    'm' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::MIN, Vec::new()))
                    }
                    'M' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::MAX, Vec::new()))
                    }
                    '0' => {
                        let literal = Operand::Literal(Literal::ZERO);
                        self.add_instruction(Instruction::new(
                            address,
                            Mnemonic::PUSH,
                            vec![literal],
                        ))
                    }
                    '1' => {
                        let literal = Operand::Literal(Literal::INFINITY);
                        self.add_instruction(Instruction::new(
                            address,
                            Mnemonic::PUSH,
                            vec![literal],
                        ))
                    }
                    '\'' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::FRAC, Vec::new()))
                    }
                    '!' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::NEG, Vec::new()))
                    }
                    '.' => {
                        self.add_instruction(Instruction::new(address, Mnemonic::POP, Vec::new()))
                    }
                    _ => {}
                }
            }
        }
    }
}
