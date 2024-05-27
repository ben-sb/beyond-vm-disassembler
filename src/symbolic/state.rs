use super::symbol::Symbol;
use std::collections::HashMap;

pub struct State {
    pos: usize,
    variables: HashMap<String, Symbol>,
}
