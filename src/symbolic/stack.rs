use super::symbol::Symbol;

#[derive(Debug)]
pub struct Stack {
    size: usize,
    elements: Vec<Symbol>,
}

impl Stack {
    // Creates a new stack.
    pub fn new() -> Stack {
        Stack {
            size: 0,
            elements: Vec::new(),
        }
    }

    /// Returns the size of the stack.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Pushes a new element to the stack.
    pub fn push(&mut self, element: Symbol) {
        self.elements.push(element);
        self.size += 1;
    }

    /// Pops from the stack.
    pub fn pop(&mut self) -> Option<Symbol> {
        if let Some(elem) = self.elements.pop() {
            self.size -= 1;
            Some(elem)
        } else {
            None
        }
    }
}
