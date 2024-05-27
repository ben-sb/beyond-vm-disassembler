use super::{instruction::Instruction, operand::Function};

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
    fn add_instruction(&self, instr: Instruction) {
        match self.current_function_index {
            Some(i) => self.functions[i].add_instruction(instr),
            None => panic!("Tried to add an instruction when there is no current function"),
        };
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
    pub fn disassemble(&self) {
        todo!()
    }
}
