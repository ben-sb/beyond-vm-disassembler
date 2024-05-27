use super::{stack::Stack, symbol::Symbol};
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pos: usize,
    variables: HashMap<String, Symbol>,
    stack: Stack,
}

impl State {
    // Creates a new state with a given pos.
    pub fn new(pos: usize) -> State {
        State {
            pos,
            variables: HashMap::new(),
            stack: Stack::new(),
        }
    }
}
