use super::{
    instruction::Instruction,
    operand::{FunctionReference, Variable},
};

#[derive(Debug)]
pub struct Function {
    address: usize,
    id: String,
    formatted_name: String,
    num_params: u8,
    instructions: Vec<Instruction>,
}

impl Function {
    /// Creates a new function.
    pub fn new(address: usize, id: String, num_params: u8) -> Function {
        let formatted_name = format!("func_{id}");
        Function {
            address,
            id: id,
            formatted_name,
            num_params,
            instructions: Vec::new(),
        }
    }

    /// Returns the ID of the function.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the instructions within the function.
    pub fn instructions(self) -> Vec<Instruction> {
        self.instructions
    }

    /// Adds an instruction to the function.
    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    /// Returns a reference to the function. Can be used for referring
    /// to calls of the function within instructions.
    pub fn get_reference(&self) -> FunctionReference {
        FunctionReference::new(self.id.clone())
    }

    /// Returns the disassembly for the function as a vector of disassembly lines.
    pub fn get_disassembly(&self) -> Vec<String> {
        let param_str = (0..self.num_params)
            .map(|i| {
                format!(
                    "param_{i}{}",
                    if i < self.num_params - 1 { ", " } else { "" }
                )
            })
            .collect::<Vec<String>>()
            .join("");
        let func_line = format!("function {}({})", self.formatted_name, param_str);

        let mut dis: Vec<String> = vec![func_line];

        for instr in &self.instructions {
            let mut line = String::from("\t");
            line.push_str(&instr.to_string());
            dis.push(line);
        }

        dis
    }
}

impl Variable for Function {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}
