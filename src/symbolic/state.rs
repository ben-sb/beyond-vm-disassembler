use crate::disassembler::instruction::{Instruction, Mnemonic};

use super::{stack::Stack, symbol::Symbol};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Status {
    Active,
    Terminated,
    Errorred,
}

#[derive(Debug)]
pub struct State {
    id: usize,
    pos: usize,
    variables: HashMap<String, Symbol>,
    stack: Stack,
    status: Status,
}

impl State {
    // Creates a new state with a given pos.
    pub fn new(id: usize, pos: usize) -> State {
        State {
            id,
            pos,
            variables: HashMap::new(),
            stack: Stack::new(),
            status: Status::Active,
        }
    }

    /// Returns the ID of the state.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the position of the state.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the status of the state.
    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Executes a single instruction.
    pub fn step(&mut self, instruction: &Instruction) {
        println!("Stepping state {}", self.id);

        match instruction.mnemonic() {
            Mnemonic::PUSH => {}
            _ => {}
        }

        self.status = Status::Terminated;
    }
}
