use std::cmp::PartialEq;
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
}