use super::instruction::Instruction;

#[derive(Debug)]
pub enum Operand {
    Literal(Literal),
    GlobalVariable(GlobalVariable),
    Parameter(Parameter),
    FunctionReference(FunctionReference),
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

    /// Returns the ID of the function.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Adds an instruction to the function.
    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    /// Returns a reference to the function. Can be used for referring
    /// to the function within instructions.
    pub fn get_reference(&self) -> FunctionReference {
        FunctionReference::new(self.id.clone())
    }
    // TODO: implement other methods
}

impl Variable for Function {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}

#[derive(Debug)]
pub struct FunctionReference {
    id: String,
    formatted_name: String,
}

impl FunctionReference {
    /// Creates a new function reference.
    pub fn new(id: String) -> FunctionReference {
        let formatted_name = format!("func_{id}");
        FunctionReference { id, formatted_name }
    }
}

impl Variable for FunctionReference {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}
