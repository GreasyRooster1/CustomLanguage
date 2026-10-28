mod tests;
mod interpret;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use serde::Serialize;
use crate::ast::{BlockStatement, Function, FunctionCall, Name, Node, Parameter, Program, Statement};

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<StackScope>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter{
            stack: vec![StackScope::new()],
        }
    }

    pub(crate) fn execute(&mut self,program: Program) -> Value {
        program.execute(self)
    }

    pub(crate) fn current_scope(&mut self) -> &mut StackScope {
        self.stack.last_mut().unwrap()
    }
    pub(crate) fn global_scope(&mut self) -> &mut StackScope {
        self.stack.first_mut().unwrap()
    }

    pub(crate) fn get_variable(&mut self, name: Name) -> Option<Value> {
        match self.current_scope().variables.get(&name){
            Some(v) => Some(v.clone()),
            None => match self.global_scope().variables.get(&name) {
                Some(v) => Some(v.clone()),
                None => None
            }
        }
    }
    pub(crate) fn get_variable_strict_scope(&self, name: Name, scope:StackScope) -> Option<Value> {
        match scope.variables.get(&name){
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}

#[derive(Debug)]
pub(crate) struct StackScope {
    variables:HashMap<Name, Value>,
}

impl StackScope {
    fn new() -> StackScope {
        StackScope{
            variables: HashMap::new(),
        }
    }
}

#[derive(Debug,Clone,Serialize)]
pub(crate) enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    #[serde(skip_serializing)]
    Function(Arc<dyn FunctionExecution>),
    NoValue
}


pub trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter) -> Value;
}

pub trait FunctionExecution: Debug {
    fn execute(&self, header:FunctionHeader, interpreter: &mut Interpreter);
}

#[derive(Debug,Clone,Serialize)]
pub struct FunctionHeader{
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) corresponding_args: Vec<Value>,
}
#[derive(Debug)]
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

#[derive(Debug)]
struct DebugFunctionExecution;
impl FunctionExecution for DebugFunctionExecution{
    fn execute(&self, header:FunctionHeader, interpreter: &mut Interpreter){
        println!("debug called with header {:#?} and state: {:?}",header,interpreter);
    }
}