mod tests;

use std::cmp::PartialEq;
use serde::Serialize;
use crate::ast::Node::{BinOp, Function, Number, Parameter};
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
            _ => {panic!("not a number")}
        }
    }

    fn parse_statement(&mut self) -> Node{
        match self.peek(){
            Token::Func => self.parse_function(),
            // Token::Loop => {}
            // Token::Name(_) => {}
            // Token::Return =>{}

            _ => {todo!()}
        }
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

        self.expect_and_get(TokenType::OpenBracket);
        let statement = self.parse_statement();
        self.expect_and_get(TokenType::CloseBracket);
        Function(Box::from(name), params, Box::new(return_type), Box::new(statement))
    }
}

#[derive(Serialize, Debug)]
pub enum Node{
    BinOp(Box<Node>, Box<Token>, Box<Node>),
    Number(NumberLiteralAssumption),
    Function(Box<Name>,Vec<Box<Node>>,Box<Name>,Box<Node>), //name, params, return type, statement
    Parameter(Box<Name>, Box<Name>), // name, type
}

fn unwrap_name(token: Token) -> Result<Name, ()>{
    match token {
        Token::Name(a)=>{
            Ok(Name(a))
        }
        _ => {Err(())}
    }
}