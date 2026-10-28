mod tests;
mod interpret;
mod value;
mod functions;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use serde::Serialize;
use strum::Display;
use crate::ast::{BlockStatement, Function, FunctionCall, Name, Node, Parameter, Program, Statement};
use crate::error::CompileError;
use crate::interpreter::value::Value;

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

    pub(crate) fn get_variable_typed(&mut self, name: Name,val_type:Value) -> Result<Option<Value>,CompileError>{
        match self.get_variable(name){
            Some(v) => {
                if matches!(v,val_type){
                    Ok(Some(val_type))
                }else{
                    Err(CompileError::WrongTypeAssumption { expected: val_type})
                }
            }
            None => Ok(None),
        }
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
        StackScope {
            variables: HashMap::new(),
        }
    }
}

pub trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter) -> Value;
}