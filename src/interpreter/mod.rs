use std::collections::HashMap;
use crate::ast::Node;

pub struct Interpreter {
    stack: Vec<Node>,
}

struct StackFrame{
    scope:HashMap<String,Variable>,
}

enum Variable{
    Float(f32),
    Int(i32),
    String(String),
}

pub trait Interpret {
    fn execute(&self);
}