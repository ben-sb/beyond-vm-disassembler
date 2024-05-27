use super::instruction::Instruction;

#[derive(Debug)]
pub enum Operand {
    Literal(Literal),
    GlobalVariable(GlobalVariable),
    Parameter(Parameter),
    FunctionReference(FunctionReference),
}

impl Operand {
    /// Returns a string representation of the operand.
    pub fn to_string(&self) -> String {
        let str = match self {
            Operand::Literal(literal) => literal.to_string(),
            Operand::GlobalVariable(var) => var.to_string(),
            Operand::Parameter(param) => param.to_string(),
            Operand::FunctionReference(func_ref) => func_ref.to_string(),
        };
        String::from(str)
    }
}

#[derive(Debug)]
pub enum Literal {
    ZERO,
    INFINITY,
}

impl Literal {
    /// Returns a string representation of the literal.
    pub fn to_string(&self) -> &str {
        match self {
            Literal::ZERO => "0",
            Literal::INFINITY => "Infinity",
        }
    }
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

    /// Returns a string representation of the global variable.
    pub fn to_string(&self) -> &str {
        &self.formatted_name
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

    /// Returns a string representation of the parameter.
    pub fn to_string(&self) -> &str {
        &self.formatted_name
    }
}

impl Variable for Parameter {
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

    /// Returns a string representation of the function reference.
    pub fn to_string(&self) -> &str {
        &self.formatted_name
    }
}

impl Variable for FunctionReference {
    /// Returns the name of the variable.
    fn name(&self) -> &str {
        &self.formatted_name
    }
}
