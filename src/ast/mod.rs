mod tests;
pub mod nodes;

pub use crate::ast::nodes::*;
use std::cmp::PartialEq;
use serde::Serialize;
use crate::ast::Node::ProgramNode;
use crate::lexer::{NumberLiteralAssumption, Token, TokenType};

#[derive(Debug, Serialize)]
pub struct Name(String);

pub struct AST{
    tokens: Vec<Token>,
    index: usize,
}

impl AST{
    fn new(tokens: Vec<Token>) -> AST{
        AST{
            tokens,
            index:0,
        }
    }

    fn peek(&self) -> &Token {
        if(self.index>=self.tokens.len()){
            return &Token::EOF;
        }
        &self.tokens[self.index]
    }

    fn eat(&mut self) -> &Token {
        self.index+=1;
        &self.tokens[self.index-1]
    }

    fn expect(&mut self, expected: TokenType){
        let token = self.eat();
        if expected!=token {
            panic!("got {:?}, expected {:?} {:?}", token, expected, expected==token);
        }
    }

    fn expect_and_get(&mut self, expected: TokenType) -> Token {
        let token = self.eat().clone();
        if expected!=&token {
            panic!("got {:?}, expected {:?}", token, expected);
        }
        token
    }

    fn parse_add_sub(&mut self) -> Node{
        let mut left = self.parse_term();
        while TokenType::Add==self.peek() || TokenType::Sub==self.peek(){
            let op = self.eat().clone();
            let right = self.parse_term();
            left = Node::BinOpNode(Box::new(BinOp{
                left,
                op,
                right
            }))
        }
        left
    }

    fn parse_term(&mut self) -> Node{
        let mut left = self.parse_gate();
        while TokenType::Mult==self.peek() || TokenType::Div==self.peek() || TokenType::Mod==self.peek(){
            let op = self.eat().clone();
            let right = self.parse_term();
            left = Node::BinOpNode(Box::new(BinOp{
                left,
                op,
                right
            }))
        }
        left
    }

    fn parse_factor(&mut self) -> Node{
        if TokenType::OpenParam == self.peek(){
            self.eat();
            let node = self.parse_expr();
            println!("{:?}", self.peek());
            self.expect(TokenType::CloseParam);
            return node;
        }
        match self.eat().clone(){
            Token::NumberLiteral(assumption)=>Node::NumberNode(Box::new(Number{
                assumption
            })),
            Token::Name(s)=>Node::IdentifierNode(Box::new(Identifier{
                name:Name(s),
            })),
            _ => {panic!("not a number")}
        }
    }
    fn parse_gate(&mut self) -> Node{
        let mut left = self.parse_factor();
        while TokenType::And==self.peek() || TokenType::Or==self.peek(){
            let op = self.eat().clone();
            let right = self.parse_term();
            left = Node::BinOpNode(Box::new(BinOp{
                left,
                op,
                right
            }))
        }
        left
    }
    fn parse_expr(&mut self) -> Node{
        let mut left = self.parse_add_sub();
        while TokenType::CheckEq==self.peek() || TokenType::CheckNotEq==self.peek() || TokenType::GreaterThan==self.peek() || TokenType::LessThan==self.peek(){
            let op = self.eat().clone();
            let right = self.parse_term();
            left = Node::BinOpNode(Box::new(BinOp{
                left,
                op,
                right
            }))
        }
        left
    }

    fn parse_range(&mut self) -> Node{
        let left = self.parse_expr();
        self.expect(TokenType::RangeSeparator);
        let right = self.parse_expr();
        let step =if TokenType::StepSeparator==self.peek() {
            self.expect(TokenType::StepSeparator);
            self.parse_expr()
        }else{
            Node::NumberNode(Box::new(Number{
                assumption:NumberLiteralAssumption::Int(1)
            }))
        };
        Node::RangeNode(Box::new(Range{
            start:left,
            stop:right,
            step,
        }))
    }

    fn parse_statement(&mut self) -> Node{
        match self.peek(){
            Token::Func => self.parse_function(),
            Token::Loop => self.parse_loop(),
            // Token::Name(_) => {}
            Token::Return => self.parse_return(),
            Token::EOF => panic!("EOF"),
            Token::If => self.parse_if(),

            _ => {panic!("not implemented {}",self.peek())}
        }
    }

    fn parse_loop(&mut self) -> Node{
        self.expect(TokenType::Loop);
        if TokenType::OpenBracket == self.peek(){
            let block = self.parse_block();
            return Node::ForeverLoopNode(Box::new(ForeverLoop{
                body:block
            }));
        }
        let name_token = self.expect_and_get(TokenType::Name);
        let name = unwrap_name(name_token).unwrap();
        let range = self.parse_range();
        let block = self.parse_block();
        Node::ForLoopNode(Box::new(ForLoop{
            name,
            range,
            body:block,
        }))
    }

    fn parse_function(&mut self) -> Node{
        self.expect(TokenType::Func);
        let name_token = self.expect_and_get(TokenType::Name);
        let name = unwrap_name(name_token).unwrap();
        self.expect(TokenType::OpenParam);

        let mut params = Vec::new();
        while TokenType::Name==self.peek() {
            let param_name_token = self.expect_and_get(TokenType::Name);
            let param_name = unwrap_name(param_name_token).unwrap();
            self.expect(TokenType::TypeSeparator);
            let param_type_token = self.expect_and_get(TokenType::Name);
            let param_type = unwrap_name(param_type_token).unwrap();
            if TokenType::Comma==self.peek() {
                self.expect(TokenType::Comma);
            }
            params.push(Parameter{
                name:param_name,
                p_type:param_type,
            });
        }

        self.expect(TokenType::CloseParam);

        let return_type = if TokenType::TypeSeparator==self.peek() {
            self.eat();
            let return_type_token = self.expect_and_get(TokenType::Name);
            unwrap_name(return_type_token).unwrap()
        }else{
            Name("None".to_string())
        };

        let block = self.parse_block();


        Node::FunctionNode(Box::from(Function{
            name,
            params,
            ret_type:return_type,
            body:block,
        }))
    }

    fn parse_block(&mut self) -> Node{
        self.expect(TokenType::OpenBracket);
        let mut statements = Vec::new();
        while TokenType::CloseBracket!=self.peek() {
           statements.push(self.parse_statement());
        }
        if statements.len()==0 {
            statements.push(Node::NopNode);
        }
        self.expect(TokenType::CloseBracket);
        Node::BlockStatementNode(Box::new(BlockStatement{
            statements
        }))
    }

    fn parse_return(&mut self) -> Node{
        self.expect(TokenType::Return);

        Node::ReturnStatementNode(Box::new(ReturnStatement {
            expression: self.parse_expr()
        }))
    }

    fn parse_if(&mut self) -> Node{
        self.expect(TokenType::If);
        let condition = self.parse_expr();
        let body = self.parse_block();
        Node::IfNode(Box::new(If{
            condition,
            body
        }))
    }

    fn parse_program(&mut self) -> Node{
        let mut statements = Vec::new();
        while TokenType::EOF!=self.peek() {
            statements.push(self.parse_statement());
        }
        Node::ProgramNode(Box::new(Program {
            statements
        }))
    }
}

#[derive(Serialize, Debug)]
pub enum Node{
    BinOpNode(Box<BinOp>),
    NumberNode(Box<Number>),
    FunctionNode(Box<Function>), //name, params, return type, block statement
    ParameterNode(Box<Parameter>), // name, type
    IdentifierNode(Box<Identifier>),
    BlockStatementNode(Box<BlockStatement>),
    ReturnStatementNode(Box<ReturnStatement>),
    ProgramNode(Box<Program>),
    ForeverLoopNode(Box<ForeverLoop>),
    ForLoopNode(Box<ForLoop>), // var, range, block
    RangeNode(Box<Range>), //start, stop, step
    IfNode(Box<If>), //condition, block
    NotNode(Box<Not>),
    NopNode,
}

fn unwrap_name(token: Token) -> Result<Name, ()>{
    match token {
        Token::Name(a)=>{
            Ok(Name(a))
        }
        _ => {Err(())}
    }
}