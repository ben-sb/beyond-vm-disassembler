use super::symbol::Symbol;

#[derive(Debug)]
pub struct Stack {
    size: StackSize,
}

impl Stack {
    // Creates a new stack.
    pub fn new() -> Stack {
        Stack {
            size: StackSize::Literal(0),
        }
    }
}

#[derive(Debug)]
enum StackSize {
    Literal(usize),
    Phi(Phi),
}

#[derive(Debug)]
pub struct Phi {
    condition: Symbol,
    consequent: Symbol,
    alternate: Symbol,
}
