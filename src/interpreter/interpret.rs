use crate::ast::{BlockStatement, Function, FunctionCall, Node, Program, Statement};
use crate::error::{CompileError, CompileResult};
use crate::error::CompileError::Unspecified;
use crate::interpreter::{Interpret, Interpreter};
use crate::interpreter::functions::FunctionHeader;
use crate::interpreter::value::{Value, ValueType};
use crate::interpreter::value::Value::NoValue;
use crate::lexer::Token::Func;

impl Interpret for Program{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        for statement in &self.statements{
            let Node::StatementNode(s) = statement else { panic!("not a statement") };
            s.execute(interpreter);
        }
        Ok(NoValue)
    }
}
impl Interpret for BlockStatement{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        for statement in &self.statements{
            let Node::StatementNode(s) = statement else { panic!("not a statement") };
            s.execute(interpreter);
        }
        Ok(NoValue)
    }
}

impl Interpret for Statement{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        match &self.statement {
            Node::FunctionNode(f) => f.execute(interpreter),
            Node::BlockStatementNode(b) => b.execute(interpreter),
            Node::FunctionCallNode(c) => c.execute(interpreter),
            // Node::ReturnStatementNode(r) => r.execute(interpreter),
            // Node::ForeverLoopNode(f) => f.execute(interpreter),
            // Node::ForLoopNode(f) => f.execute(interpreter),
            // Node::IfNode(i) => i.execute(interpreter),
            _ => Ok(NoValue)
        }
    }
}

impl Interpret for Function{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let Node::BlockStatementNode(body) = self.body.clone() else { panic!("not a statement") };
        interpreter.global_scope().variables.insert(self.name.clone(), *body);
        Ok(NoValue)
    }
}

impl Interpret for FunctionCall{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let possible_function = interpreter.get_variable_typed(self.name, ValueType::Function)?;
        let function = match possible_function {
            Some(f) => f,
            None => Err(CompileError::ValueNotFound)?
        };
        let Value::Function(function) = function else {return Err(Unspecified)};
        let header = FunctionHeader::new(self.args,function.parameters);
        function.execute()
    }
}