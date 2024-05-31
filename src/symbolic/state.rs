use crate::{
    disassembler::{
        instruction::{Instruction, Mnemonic},
        operand::Operand,
    },
    symbolic::symbol::{
        BinaryExpressionSymbol, BinaryOperator, CallExpressionSymbol, IdentifierSymbol,
        LiteralSymbol, UnaryExpressionSymbol, UnaryOperator,
    },
};

use super::{stack::Stack, symbol::Symbol};
use std::{collections::HashMap, error::Error};

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

    /// Pushes parameters to the stack before the symbolic execution starts.
    pub fn push_params(&mut self, num_params: usize) {
        for i in 0..num_params {
            let param = IdentifierSymbol::new(format!("param_{i}"));
            self.stack.push(Symbol::IdentifierSymbol(param));
        }
    }

    /// Helper method to access the ith operand from an instruction.
    fn get_operand<'instr>(
        &mut self,
        instruction: &'instr Instruction,
        index: usize,
    ) -> Result<&'instr Operand, Box<dyn Error>> {
        if let Some(operand) = instruction.operands().get(index) {
            Ok(operand)
        } else {
            self.status = Status::Errorred;
            Err("Expected operand at index {index} but got None, bad disassembly".into())
        }
    }

    /// Helper method to pop from the stack.
    fn pop(&mut self) -> Result<Symbol, Box<dyn Error>> {
        self.stack
            .pop()
            .ok_or("Got empty stack when operand expected".into())
    }

    /// Steps the state by execution a single instruction.
    pub fn step(&mut self, instruction: &Instruction) {
        match self.execute(instruction) {
            Ok(_) => {}
            Err(e) => {
                println!("{e}");
                self.status = Status::Errorred
            }
        }
    }

    /// Symbolically executes an instruction.
    fn execute(&mut self, instruction: &Instruction) -> Result<(), Box<dyn Error>> {
        self.pos += 1;

        match instruction.mnemonic() {
            Mnemonic::PUSH => {
                let operand = self.get_operand(instruction, 0)?;
                let symbol = Symbol::from(operand);
                self.stack.push(symbol);
            }
            Mnemonic::POP => {
                self.pop()?;
            }
            Mnemonic::ADD => {
                let left = self.pop()?;
                let right = self.pop()?;
                // TODO: simplify logic for literals
                let sum = BinaryExpressionSymbol::new(
                    BinaryOperator::Add,
                    Box::new(left),
                    Box::new(right),
                );
                self.stack.push(Symbol::BinaryExpressionSymbol(sum));
            }
            Mnemonic::SUB => {
                let left = self.pop()?;
                let right = self.pop()?;
                // TODO: simplify logic for literals
                let sum = BinaryExpressionSymbol::new(
                    BinaryOperator::Subtract,
                    Box::new(left),
                    Box::new(right),
                );
                self.stack.push(Symbol::BinaryExpressionSymbol(sum));
            }
            Mnemonic::MUL => {
                let left = self.pop()?;
                let right = self.pop()?;
                // TODO: simplify logic for literals
                let product = BinaryExpressionSymbol::new(
                    BinaryOperator::Multiply,
                    Box::new(left),
                    Box::new(right),
                );
                self.stack.push(Symbol::BinaryExpressionSymbol(product));
            }
            Mnemonic::DIV => {
                let left = self.pop()?;
                let right = self.pop()?;
                // TODO: simplify logic for literals
                let div = BinaryExpressionSymbol::new(
                    BinaryOperator::Divide,
                    Box::new(left),
                    Box::new(right),
                );
                self.stack.push(Symbol::BinaryExpressionSymbol(div));
            }
            // TODO: handle min and max
            Mnemonic::FRAC => {
                let one = Symbol::LiteralSymbol(LiteralSymbol::One);
                let operand = self.pop()?;
                // TODO: simplify logic for literals
                let frac = BinaryExpressionSymbol::new(
                    BinaryOperator::Divide,
                    Box::new(one),
                    Box::new(operand),
                );
                self.stack.push(Symbol::BinaryExpressionSymbol(frac));
            }
            Mnemonic::NEG => {
                let argument = self.pop()?;
                // TODO: simplify logic for literals
                let result = UnaryExpressionSymbol::new(UnaryOperator::Not, Box::new(argument));
                self.stack.push(Symbol::UnaryExpressionSymbol(result));
            }
            Mnemonic::MIN | Mnemonic::MAX => {
                let left = self.pop()?;
                let right = self.pop()?;
                let arguments = vec![left, right];

                let callee_name = instruction.mnemonic().to_string();
                let callee = IdentifierSymbol::new(callee_name);

                let call = CallExpressionSymbol::new(
                    Box::new(Symbol::IdentifierSymbol(callee)),
                    arguments,
                );
                self.stack.push(Symbol::CallExpressionSymbol(call))
            }
            Mnemonic::CALL => {
                let operand = self.get_operand(instruction, 0)?;
                let callee = Symbol::from(operand);

                let mut arguments = Vec::new();
                let mut i = 0;
                while i < 8 && self.stack.size() > 0 {
                    if let Some(operand) = self.stack.pop() {
                        arguments.push(operand);
                    }
                    i += 1;
                }

                let call = CallExpressionSymbol::new(Box::new(callee), arguments);
                println!("{}", call);
                self.stack.push(Symbol::CallExpressionSymbol(call))
            }
            Mnemonic::RET => {
                println!("{}", self.stack);
                self.status = Status::Terminated;
            }
        }

        Ok(())
    }
}
