use std::cmp::PartialEq;
use crate::ast::Node::BinOp;
use crate::lexer::{Token, TokenType};

pub struct AST{
    tokens: Vec<Token>,
    index: usize,
}

impl AST{
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
        let mut left = self.parse_term();
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
            let mut node = self.parse_expr();
            self.expect(TokenType::CloseParam);
            return node;
        }
        return self.parse_expr();
    }
}

pub enum Node{
    BinOp(Box<Node>, Box<Token>, Box<Node>),
    Number()
}