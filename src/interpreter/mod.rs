mod tests;

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

impl Interpret for Program{
    fn execute(&self, interpreter: &mut Interpreter) {
        for statement in &self.statements{
            let Node::StatementNode(s) = statement else { return };
            s.execute(interpreter);
        }
    }
}
impl Interpret for BlockStatement{
    fn execute(&self, interpreter: &mut Interpreter) {
        for statement in &self.statements{
            let Node::StatementNode(s) = statement else { return };
            s.execute(interpreter);
        }
    }
}

impl Interpret for Statement{
    fn execute(&self, interpreter: &mut Interpreter) {
        match &self.statement {
            Node::FunctionNode(f) => f.execute(interpreter),
            Node::BlockStatementNode(b) => b.execute(interpreter),
            Node::FunctionCallNode(c) => c.execute(interpreter),
            // Node::ReturnStatementNode(r) => r.execute(interpreter),
            // Node::ForeverLoopNode(f) => f.execute(interpreter),
            // Node::ForLoopNode(f) => f.execute(interpreter),
            // Node::IfNode(i) => i.execute(interpreter),
            _ => {}
        }
    }
}

impl Interpret for Function{
    fn execute(&self, interpreter: &mut Interpreter) {
        let Node::BlockStatementNode(body) = self.body.clone() else { return };
        interpreter.global_scope.functions.insert(self.name.clone(), *body);
    }
}

impl Interpret for FunctionCall{
    fn execute(&self, interpreter: &mut Interpreter) {
        let function = interpreter.global_scope.functions.get(&self.name).unwrap().clone();
        function.execute(interpreter);
    }
}