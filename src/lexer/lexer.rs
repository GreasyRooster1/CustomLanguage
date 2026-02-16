use crate::lexer::{Token, TokenRule};
use crate::lexer::tokens::*;

pub(crate) fn get_matching_tokens(string:String, rules:Vec<Box<dyn TokenRule>>) -> Vec<Token> {
    let mut tokens = vec![];
    
    
    
    tokens
}

pub(crate) fn parse(){
    let rules = alloc_rules();

}

pub(crate) fn alloc_rules() -> Vec<Box<dyn TokenRule>>{
    vec![
        Box::new(FuncRule),
    ]
}