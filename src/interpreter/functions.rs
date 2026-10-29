use std::env::Args;
use std::fmt::Debug;
use serde::Serialize;
use crate::ast::{BlockStatement, Node, Parameter};
use crate::error::{CompileError, CompileResult};
use crate::error::CompileError::{ArgsDontMatchSig, Unspecified};
use crate::interpreter::{Interpret, Interpreter, StackScope};
use crate::interpreter::value::{Value, ValueType};
use crate::interpreter::value::Value::NoValue;

#[derive(Debug,Clone,Serialize)]
pub struct FunctionHeader{
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) corresponding_args: Vec<Value>,
}

impl FunctionHeader {
    pub fn new(args:Vec<Value>,parameters: Vec<Parameter>) -> CompileResult<Self> {
        if args.len()!=parameters.len(){
            return Err(ArgsDontMatchSig)
        }
        for i in 0..parameters.len(){
            if args[i]!=ValueType::from(parameters[i].p_type.clone()) {
                return Err(ArgsDontMatchSig)
            }
        }
        Ok(FunctionHeader{
            parameters,
            corresponding_args:args,
        })
    }
}


pub trait FunctionExecution: Debug {
    fn call(&self, header:FunctionHeader, interpreter: &mut Interpreter) -> CompileResult<Value>;
}

#[derive(Debug)]
pub struct BlockFunctionExecution{
    pub(crate) block: BlockStatement,
}
impl FunctionExecution for BlockFunctionExecution{
    fn call(&self, header:FunctionHeader, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let mut scope = StackScope::new();
        for i in 0..header.parameters.len() {
            scope.variables.insert(header.parameters[i].name.clone(), header.corresponding_args[i].clone());
        }
        println!("new scope: {:#?}",scope);
        interpreter.stack.push(scope);

        self.block.execute(interpreter)
    }
}

#[derive(Debug)]
pub struct DebugFunctionExecution;
impl FunctionExecution for DebugFunctionExecution{
    fn call(&self, header:FunctionHeader, interpreter: &mut Interpreter) -> CompileResult<Value>{
        println!("debug called with header {:#?} and state: {:?}",header,interpreter);
        Ok(NoValue)
    }
}

#[derive(Debug)]
pub struct InsulateFunctionExecution;
impl FunctionExecution for InsulateFunctionExecution{
    fn call(&self, header:FunctionHeader, interpreter: &mut Interpreter) -> CompileResult<Value>{
        let Value::ShortCircuit(val) = header.corresponding_args[0].clone() else { return Err(Unspecified) };
        Ok(*val)
    }
}