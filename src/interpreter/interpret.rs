use crate::ast::{BlockStatement, Function, FunctionCall, Node, Program, Statement};
use crate::interpreter::{Interpret, Interpreter};
use crate::interpreter::value::{Value, ValueType};
use crate::interpreter::value::Value::NoValue;

impl Interpret for Program{
    fn execute(&self, interpreter: &mut Interpreter) -> Value {
        for statement in &self.statements{
            let Node::StatementNode(s) = statement else { panic!("not a statement") };
            s.execute(interpreter);
        }
        NoValue
    }
}
impl Interpret for BlockStatement{
    fn execute(&self, interpreter: &mut Interpreter) -> Value {
        for statement in &self.statements{
            let Node::StatementNode(s) = statement else { panic!("not a statement") };
            s.execute(interpreter);
        }
        NoValue
    }
}

impl Interpret for Statement{
    fn execute(&self, interpreter: &mut Interpreter) -> Value {
        match &self.statement {
            Node::FunctionNode(f) => f.execute(interpreter),
            Node::BlockStatementNode(b) => b.execute(interpreter),
            Node::FunctionCallNode(c) => c.execute(interpreter),
            // Node::ReturnStatementNode(r) => r.execute(interpreter),
            // Node::ForeverLoopNode(f) => f.execute(interpreter),
            // Node::ForLoopNode(f) => f.execute(interpreter),
            // Node::IfNode(i) => i.execute(interpreter),
            _ => NoValue
        }
    }
}

impl Interpret for Function{
    fn execute(&self, interpreter: &mut Interpreter) -> Value {
        let Node::BlockStatementNode(body) = self.body.clone() else { panic!("not a statement") };
        interpreter.global_scope().variables.insert(self.name.clone(), *body);
        NoValue
    }
}

impl Interpret for FunctionCall{
    fn execute(&self, interpreter: &mut Interpreter) -> Value {
        let function = interpreter.get_variable_typed(self.name,ValueType::Function);
        function.execute(interpreter);
        NoValue
    }
}