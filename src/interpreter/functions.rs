use std::fmt::Debug;
use serde::Serialize;
use crate::ast::{BlockStatement, Parameter};
use crate::interpreter::{Interpret, Interpreter, StackScope};
use crate::interpreter::value::Value;

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