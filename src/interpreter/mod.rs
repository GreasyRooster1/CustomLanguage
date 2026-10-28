mod tests;
mod interpret;
pub(crate) mod value;
mod functions;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use serde::Serialize;
use strum::Display;
use crate::ast::{BlockStatement, Function, FunctionCall, Name, Node, Parameter, Program};
use crate::error::{CompileError, CompileResult};
use crate::interpreter::functions::DebugFunctionExecution;
use crate::interpreter::value::{FunctionValue, Value, ValueType};

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<StackScope>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter{
            stack: vec![StackScope::global()],
        }
    }

    pub(crate) fn execute(&mut self,program: Program) -> CompileResult<Value> {
        program.execute(self)
    }

    pub(crate) fn current_scope(&mut self) -> &mut StackScope {
        self.stack.last_mut().unwrap()
    }
    pub(crate) fn global_scope(&mut self) -> &mut StackScope {
        self.stack.first_mut().unwrap()
    }

    pub(crate) fn get_variable_typed(&mut self, name: &Name, val_type:ValueType) -> Result<Option<Value>,CompileError>{
        match self.get_variable(name){
            Some(v) => {
                if v==val_type {
                    Ok(Some(v))
                }else{
                    Err(CompileError::WrongTypeAssumption { expected: val_type})
                }
            }
            None => Ok(None),
        }
    }
    pub(crate) fn get_variable(&mut self, name: &Name) -> Option<Value> {
        match self.current_scope().variables.get(&name){
            Some(v) => Some(v.clone()),
            None => match self.global_scope().variables.get(&name) {
                Some(v) => Some(v.clone()),
                None => None
            }
        }
    }
    pub(crate) fn get_variable_strict_scope(&self, name: &Name, scope:StackScope) -> Option<Value> {
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
    fn global() -> StackScope {
        let mut scope:HashMap<Name, Value> = HashMap::new();
        scope.insert(Name("debug".to_string()),Value::Function(FunctionValue{
            executor: Arc::new(DebugFunctionExecution),
            parameters: vec![Parameter{
                name: Name("arg".to_string()),
                p_type: Name("str".to_string()),
            }],
            return_value: Name("none".to_string()),
        }));
        StackScope {
            variables: scope,
        }
    }
}

pub(crate) trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value>;
}