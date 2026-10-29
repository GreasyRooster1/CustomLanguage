use std::cmp::{Ordering, PartialOrd};
use std::sync::Arc;
use std::thread::scope;
use crate::ast::{Assignment, BinOp, BlockStatement, ForLoop, ForeverLoop, Function, FunctionCall, Identifier, If, Node, Number, Program, ReturnStatement, ShortStatement};
use crate::error::{CompileError, CompileResult};
use crate::error::CompileError::{InvalidOperation, Unspecified, ValueNotFound};
use crate::interpreter::{Interpret, Interpreter};
use crate::interpreter::functions::{BlockFunctionExecution, FunctionHeader};
use crate::interpreter::value::{FunctionValue, Value, ValueType};
use crate::interpreter::value::Value::{NoValue, ShortCircuit};
use crate::lexer::{NumberLiteralAssumption, Token};
use crate::lexer::Token::Func;

impl Interpret for Program{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        for statement in &self.statements{
            statement.execute(interpreter)?;
        }
        Ok(NoValue)
    }
}
impl Interpret for BlockStatement{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        for statement in &self.statements{
            let val = statement.execute(interpreter)?;
            if val==ValueType::ShortCircuit{
                return Ok(val);
            }
        }
        Ok(NoValue)
    }
}

impl Interpret for Function{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let Node::BlockStatementNode(body) = self.body.clone() else { panic!("not a statement") };

        let func = FunctionValue{
            executor: Arc::new(BlockFunctionExecution { block: *body }),
            parameters: self.params.clone(),
            return_value: self.ret_type.clone(),
        };

        if !self.is_anon {
        interpreter.global_scope().variables.insert(self.name.clone(), Value::Function(func.clone()));
        }
        Ok(Value::Function(func))
    }
}

impl Interpret for FunctionCall{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let possible_function = interpreter.get_variable_typed(&self.name, ValueType::Function)?;
        let function = match possible_function {
            Some(f) => f,
            None => Err(CompileError::ValueNotFound)?
        };
        let Value::Function(function) = function else { return Err(Unspecified) };

        let mut arg_values = vec![];
        for arg in &self.args {
            arg_values.push(arg.execute(interpreter)?);
        }

        let header = FunctionHeader::new(arg_values, function.parameters)?;
        let result = function.executor.call(header, interpreter)?;
        if result==ValueType::ShortCircuit{
            let Value::ShortCircuit(mut val) = result.clone() else { return Err(Unspecified) };
            println!("val: {:#?},{:#?}",val,result);
            Ok(*val)
        }else{
            Ok(result)
        }
    }
}

impl Interpret for ForeverLoop{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        loop{
            self.body.execute(interpreter)?;
        }
    }
}
impl Interpret for ForLoop{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let Node::RangeNode(range_node) = &self.range else{ return Err(Unspecified) };
        let Value::Int(start) = range_node.start.execute(interpreter)? else{ return Err(Unspecified) };
        let Value::Int(stop) = range_node.stop.execute(interpreter)? else{ return Err(Unspecified) };
        let Value::Int(step) = range_node.step.execute(interpreter)? else{ return Err(Unspecified) };
        interpreter.current_scope().variables.insert(self.name.clone(),Value::Int(start));
        for i in (start..stop).step_by(step as usize) {
            interpreter.current_scope().variables.insert(self.name.clone(),Value::Int(i));
            self.body.execute(interpreter)?;
        }
        Ok(NoValue)
    }
}

impl Interpret for If{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let val =  self.condition.execute(interpreter)?;
        if self.condition.execute(interpreter)?==ValueType::Bool{
            let Value::Bool(b) = val else {return Err(CompileError::IfConditionNotBool)};
            if b{
                Ok(self.body.execute(interpreter)?)
            }else{
                Ok(NoValue)
            }
        }else{
            Err(CompileError::IfConditionNotBool)?
        }
    }
}

impl Interpret for String{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        Ok(Value::String(self.clone()))
    }
}

impl Interpret for Number{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        Ok(match self.assumption{
            NumberLiteralAssumption::Float(f) => Value::Float(f),
            NumberLiteralAssumption::Int(i) => Value::Int(i)
        })
    }
}

impl Interpret for Identifier{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        match interpreter.get_variable(&self.name){
            Some(val) => Ok(val.clone()),
            None => Err(ValueNotFound)
        }
    }
}

impl Interpret for BinOp{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let a = self.left.execute(interpreter)?;
        let b = self.right.execute(interpreter)?;
        if (a==ValueType::Int||a==ValueType::Float) && (b==ValueType::Int||b==ValueType::Float){
            match self.op {
                Token::Add => a+b,
                Token::Sub => a-b,
                Token::Mult => a*b,
                Token::Div => a/b,
                Token::Mod => a%b,
                Token::GreaterThan => Ok(Value::Bool(a>b)),
                Token::LessThan => Ok(Value::Bool(a<b)),
                Token::GreaterThanEq => Ok(Value::Bool(a>=b)),
                Token::LessThanEq => Ok(Value::Bool(a<=b)),
                Token::CheckEq => Ok(Value::Bool(a==b)),
                Token::CheckNotEq => Ok(Value::Bool(a!=b)),
                _ => Err(Unspecified)
            }
        }else if a==ValueType::Bool && b==ValueType::Bool {
            let Value::Bool(a_bool) = a else { return Err(CompileError::Unspecified) };
            let Value::Bool(b_bool) = b else { return Err(CompileError::Unspecified) };
            match self.op{
                Token::And => Ok(Value::Bool(a_bool && b_bool)),
                Token::Or => Ok(Value::Bool(a_bool || b_bool)),
                _ => Err(Unspecified)
            }
        }else{
            Err(InvalidOperation)
        }
    }
}

impl Interpret for Assignment{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let val = self.value.execute(interpreter)?;
        interpreter.current_scope().variables.insert(self.name.clone(),val);
        Ok(NoValue)
    }
}

impl Interpret for ReturnStatement{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let val = self.expression.execute(interpreter)?;
        interpreter.stack.pop();
        Ok(ShortCircuit(Box::new(val)))
    }
}
impl Interpret for ShortStatement{
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        let val = self.expression.execute(interpreter)?;
        Ok(ShortCircuit(Box::new(val)))
    }
}

impl Interpret for Node {
    fn execute(&self, interpreter: &mut Interpreter) -> CompileResult<Value> {
        match self {
            Node::BinOpNode(n)          => n.execute(interpreter),
            Node::NumberNode(n)         => n.execute(interpreter),
            Node::StringNode(n)         => n.execute(interpreter),
            Node::FunctionNode(n)       => n.execute(interpreter),
            //Node::ParameterNode(n)      => n.execute(interpreter),
            Node::IdentifierNode(n)     => n.execute(interpreter),
            Node::BlockStatementNode(n) => n.execute(interpreter),
            Node::ReturnStatementNode(n)=> n.execute(interpreter),
            Node::ShortStatementNode(n) => n.execute(interpreter),
            Node::ProgramNode(n)        => n.execute(interpreter),
            Node::ForeverLoopNode(n)    => n.execute(interpreter),
            Node::ForLoopNode(n)        => n.execute(interpreter),
            //Node::RangeNode(n)          => n.execute(interpreter),
            Node::IfNode(n)             => n.execute(interpreter),
            //Node::NotNode(n)            => n.execute(interpreter),
            Node::FunctionCallNode(n)   => n.execute(interpreter),
            Node::AssignmentNode(n)     => n.execute(interpreter),
            Node::NopNode               => Ok(Value::NoValue),
            _ => Ok(Value::NoValue),
        }
    }
}