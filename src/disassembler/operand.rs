use super::instruction::Instruction;

#[derive(Debug)]
pub enum Operand {
    Literal(Literal),
    GlobalVariable(GlobalVariable),
    Parameter(Parameter),
}

#[derive(Debug)]
pub enum Literal {
    ZERO,
    INFINITY,
}

pub trait Variable {
    /// Returns the name of the variable.
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct GlobalVariable {
    name: String,
    formatted_name: String,
}

impl GlobalVariable {
    /// Creates a new global variable.
    pub fn new(name: String) -> GlobalVariable {
        let formatted_name = format!("global_{name}");
        GlobalVariable {
            name,
            formatted_name,
        }
    }
}

impl Variable for GlobalVariable {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}

#[derive(Debug)]
pub struct Parameter {
    index: usize,
    formatted_name: String,
}

impl Parameter {
    /// Creates a new parameter.
    pub fn new(index: usize) -> Parameter {
        let formatted_name = format!("param_{index}");
        Parameter {
            index,
            formatted_name,
        }
    }
}

impl Variable for Parameter {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}

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

    /// Adds an instruction to the function.
    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    // TODO: implement other methods
}

impl Variable for Function {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}
