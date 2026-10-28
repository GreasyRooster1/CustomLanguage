mod tests;

use std::collections::HashMap;
use crate::ast::{BlockStatement, Function, Name, Node, Program};

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

    pub(crate) fn execute(mut self,program: Program) {
        program.execute(&mut self)
    }
}

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

enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
}

pub trait Interpret {
    fn execute(&self, interpreter: &mut Interpreter);
}

impl Interpret for Program{
    fn execute(&self, interpreter: &mut Interpreter) {
        for block in &self.statements{
            let Node::BlockStatementNode(body) = block else { return };
            body.execute(interpreter);
        }
    }
}
impl Interpret for BlockStatement{
    fn execute(&self, interpreter: &mut Interpreter) {
        for block in &self.statements{
            match block{
                Node::FunctionNode(f) => f.execute(interpreter),
                Node::BlockStatementNode(b) => b.execute(interpreter),
                // Node::ReturnStatementNode(r) => r.execute(interpreter),
                // Node::ForeverLoopNode(f) => f.execute(interpreter),
                // Node::ForLoopNode(f) => f.execute(interpreter),
                // Node::IfNode(i) => i.execute(interpreter),
                _ => {}
            }
        }
    }
}

impl Interpret for Function{
    fn execute(&self, interpreter: &mut Interpreter) {
        let Node::BlockStatementNode(body) = self.body.clone() else { return };
        interpreter.global_scope.functions.insert(self.name.clone(), *body);
    }
}