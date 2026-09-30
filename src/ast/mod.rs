mod tests;

use std::cmp::PartialEq;
use serde::Serialize;
use crate::ast::Node::{BinOp, BlockStatement, Function, Identifier, Number, Parameter, Program, ReturnStatement};
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

    fn parse_expr(&mut self) -> Node{
        let mut left = self.parse_term();
        while TokenType::Add==self.peek() || TokenType::Sub==self.peek(){
            let op = self.eat().clone();
            let right = self.parse_term();
            left = BinOp(Box::from(left), Box::new(op), Box::new(right));
        }
        left
    }

    fn parse_term(&mut self) -> Node{
        let mut left = self.parse_factor();
        while TokenType::Mult==self.peek() || TokenType::Div==self.peek() || TokenType::Mod==self.peek(){
            let op = self.eat().clone();
            let right = self.parse_term();
            left = BinOp(Box::from(left), Box::new(op), Box::new(right));
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
            Token::NumberLiteral(a)=>Number(a),
            Token::Name(s)=>Identifier(Box::from(Name(s))),
            _ => {panic!("not a number")}
        }
    }

    fn parse_statement(&mut self) -> Node{
        match self.peek(){
            Token::Func => self.parse_function(),
            Token::Loop => self.parse_loop(),
            // Token::Name(_) => {}
            Token::Return => self.parse_return(),
            Token::EOF => panic!("EOF"),

            _ => {todo!()}
        }
    }
    
    fn parse_loop(&mut self) -> Node{
        self.expect(TokenType::Loop);
        let name_token = self.expect_and_get(TokenType::Name);
        let name = unwrap_name(name_token).unwrap();
        self.expect(TokenType::OpenParam);
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
            params.push(Box::new(Parameter(Box::new(param_name),Box::new(param_type))));
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


        Function(Box::from(name), params, Box::new(return_type), Box::new(block))
    }

    fn parse_block(&mut self) -> Node{
        self.expect(TokenType::OpenBracket);
        let mut statements = Vec::new();
        while TokenType::CloseBracket!=self.peek() {
           statements.push(Box::new(self.parse_statement()));
        }
        self.expect(TokenType::CloseBracket);
        BlockStatement(statements)
    }

    fn parse_return(&mut self) -> Node{
        self.expect(TokenType::Return);

        ReturnStatement(Box::new(self.parse_expr()))
    }
    
    fn parse_program(&mut self) -> Node{
        let mut statements = Vec::new();
        while TokenType::EOF!=self.peek() {
            statements.push(Box::new(self.parse_statement()));
        }
        Program(statements)
    }
}

#[derive(Serialize, Debug)]
pub enum Node{
    BinOp(Box<Node>, Box<Token>, Box<Node>),
    Number(NumberLiteralAssumption),
    Function(Box<Name>,Vec<Box<Node>>,Box<Name>,Box<Node>), //name, params, return type, block statement
    Parameter(Box<Name>, Box<Name>), // name, type
    Identifier(Box<Name>),
    BlockStatement(Vec<Box<Node>>),
    ReturnStatement(Box<Node>),
    Program(Vec<Box<Node>>),
    ForeverLoop(Box<Node>),
    ForLoop(Box<Node>, Box<Node>, Box<Node>), // var, range, block
    Range(Box<Node>, Box<Node>, Box<Node>), //start, stop, step
}

fn unwrap_name(token: Token) -> Result<Name, ()>{
    match token {
        Token::Name(a)=>{
            Ok(Name(a))
        }
        _ => {Err(())}
    }
}