mod tests;
mod interpret;

use std::collections::HashMap;
use serde::Serialize;
use crate::ast::{BlockStatement, Function, FunctionCall, Name, Node, Parameter, Program, Statement};

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

    pub(crate) fn execute(&mut self,program: Program) -> Value {
        program.execute(self)
    }

    pub(crate) fn current_stack(&mut self) -> &mut StackScope {
        self.stack.last_mut().unwrap()
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

#[derive(Debug,Clone,Serialize)]
enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    NoValue
}

pub trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter) -> Value;
}

pub trait FunctionExecution{
    fn execute(&self, header:FunctionHeader, interpreter: &mut Interpreter);
}

#[derive(Debug,Clone,Serialize)]
pub struct FunctionHeader{
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) corresponding_args: Vec<Value>,
}
struct BlockFunctionExecution{
    block: BlockStatement,
}
impl FunctionExecution for BlockFunctionExecution{
    fn execute(&self, header:FunctionHeader, interpreter: &mut Interpreter){
        let mut scope = StackScope::new();
        for i in 0..header.parameters.len() {
            scope.variables.insert(header.parameters[i].name.clone(), header.corresponding_args[i].clone());
        }
        interpreter.stack.push(scope);

        self.block.execute(interpreter);
    }
}

struct DebugFunctionExecution;
impl FunctionExecution for DebugFunctionExecution{
    fn execute(&self, header:FunctionHeader, interpreter: &mut Interpreter){
        println!("debug called with header {:#?} and state: {:?}",header,interpreter);
    }
}