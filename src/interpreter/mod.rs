use std::collections::HashMap;
use crate::ast::{BlockStatement, Node, Program};

pub struct Interpreter {
    stack: Vec<StackScope>,
    ast: Program,
    global_scope: StackScope,
}

struct StackScope {
    scope:HashMap<String, Value>,
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
                Node::FunctionNode(f) => f.execute(),
                Node::BlockStatementNode(b) => b.execute(),
                Node::ReturnStatementNode(r) => r.execute(),
                Node::ForeverLoopNode(f) => f.execute(),
                Node::ForLoopNode(f) => f.execute(),
                Node::IfNode(i) => i.execute(),
                _ => {}
            }
        }
    }
}