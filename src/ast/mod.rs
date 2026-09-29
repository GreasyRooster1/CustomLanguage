mod tests;

use std::cmp::PartialEq;
use crate::ast::Node::{BinOp, Number};
use crate::lexer::{NumberLiteralAssumption, Token, TokenType};

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
        &self.tokens[self.index]
    }

    fn eat(&mut self) -> &Token {
        self.index+=1;
        &self.tokens[self.index-1]
    }

    fn expect(&mut self, expected: TokenType){
        let token = self.eat();
        if expected==token {
            panic!("Expected something, got {:?}", token);
        }
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
            Token::Func => {}
            Token::Loop => {}
            Token::Name(_) => {}
            Token::Return =>{}
            _ => {}
        }
    }
}

pub enum Node{
    BinOp(Box<Node>, Box<Token>, Box<Node>),
    Number(NumberLiteralAssumption)
}