mod tests;
mod interpret;

use std::collections::HashMap;
use crate::ast::{BlockStatement, Function, FunctionCall, Name, Node, Program, Statement};

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<StackScope>,
    global_scope: StackScope,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter{
            stack: vec![],
            global_scope: StackScope::new(),
        }
    }

    pub(crate) fn execute(&mut self,program: Program) {
        program.execute(self)
    }
}

#[derive(Debug)]
struct StackScope {
    variables:HashMap<Name, Value>,
    functions:HashMap<Name, BlockStatement>,
}

impl StackScope {
    fn new() -> StackScope {
        StackScope{
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
}

pub trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter);
}