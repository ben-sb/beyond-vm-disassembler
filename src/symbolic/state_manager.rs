use std::collections::HashSet;

use crate::disassembler::{function::Function, instruction::Instruction};

use super::state::{State, Status};

#[derive(Debug)]
pub struct StateManager {
    instructions: Vec<Instruction>,
    states: Vec<State>,
    active_state_ids: HashSet<usize>,
    terminated_state_ids: HashSet<usize>,
}

impl StateManager {
    /// Creates a new state manager.
    pub fn new(function: Function) -> StateManager {
        let entry_state = State::new(0, 0);
        let mut active_state_ids = HashSet::new();
        active_state_ids.insert(entry_state.id());

        StateManager {
            instructions: function.instructions(),
            states: vec![entry_state],
            active_state_ids,
            terminated_state_ids: HashSet::new(),
        }
    }

    /// Keeps simulating until all states have terminated.
    pub fn explore(&mut self) {
        loop {
            let mut ids_to_remove = Vec::new();

            for id in &self.active_state_ids {
                let state = self
                    .states
                    .get_mut(*id)
                    .expect("Unexpected error when exploring states");
                if let Some(instruction) = self.instructions.get(state.pos()) {
                    state.step(instruction);

                    match *state.status() {
                        Status::Active => (),
                        _ => ids_to_remove.push(*id),
                    }
                } else {
                    ids_to_remove.push(*id);
                    self.terminated_state_ids.insert(*id);
                }
            }

            for id in ids_to_remove {
                self.active_state_ids.remove(&id);
            }

            if self.active_state_ids.len() == 0 {
                break;
            }
        }
    }
}
