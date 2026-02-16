use crate::lexer::{Token, TokenRule};
use crate::lexer::tokens::*;

fn get_matching_tokens(string:String, rules:Vec<Box<dyn TokenRule>>) -> Vec<Token> {
    let mut tokens = vec![];
    
    
    
    tokens
}

fn parse(){
    let rules = alloc_rules();

}

fn alloc_rules() -> Vec<Box<dyn TokenRule>>{
    vec![
        Box::new(FuncRule),
    ]
}