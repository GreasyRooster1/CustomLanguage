use crate::ast::{BlockStatement, Function, FunctionCall, Node, Program, Statement};
use crate::interpreter::{Interpret, Interpreter};

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