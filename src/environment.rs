use crate::tokens::{Token, TokenType};
use std::collections::HashMap;
use std::boxed::Box;

#[derive(Clone, Debug)]
pub struct Environ{
    values: HashMpa<String, TokenType>,
    pub enclosing: Option<Box<Environ>>
}

impl Environment {
    // new
    // extend
    // define
    // assign
    // get
    // contains
}
