mod tests;
mod interpret;

use std::collections::HashMap;
use std::mem::type_info::Type;
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
    NoValue
}

pub trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter) -> Value;
}

pub trait FunctionExecution{
    fn execute(&self, args:Vec<Node>, params:Vec<Parameter>, interpreter: &mut Interpreter);
}

pub struct FunctionHeader{
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) corresponding_args: Vec<Value>,
}
struct BlockFunctionExecution{
    block: BlockStatement,
}
impl FunctionExecution for BlockFunctionExecution{
    fn execute(&self, args:Vec<Node>, params:Vec<Parameter>, interpreter: &mut Interpreter){
        let mut scope = StackScope::new();
        scope.variables.insert();
        interpreter.stack.push(scope);

        self.block.execute(interpreter);
    }
}

struct DebugFunctionExecution;
impl FunctionExecution for DebugFunctionExecution{
    fn execute(&self, args:Vec<Node>, params:Vec<Parameter>, interpreter: &mut Interpreter){
        todo!()
    }
}