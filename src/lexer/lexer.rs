use std::intrinsics::mir::Return;
use crate::lexer::{Token, TokenRule};
use crate::lexer::Token::{OpenParam, TypeSeparator};
use crate::lexer::tokens::*;

pub(crate) fn get_matching_tokens(string:String, rules:Vec<Box<dyn TokenRule>>) -> Vec<Token> {
    let mut tokens = vec![];
    
    for rule in rules {
        if rule.check(&string) {
            tokens.push(rule.get_token(&string))
        }
    }
    
    tokens
}

pub(crate) fn parse(){
    let rules = alloc_rules();

}

pub(crate) fn alloc_rules() -> Vec<Box<dyn TokenRule>>{
    vec![
        Box::new(FuncRule),
        Box::new(LoopRule),

        Box::new(NumberLiteralRule),
        Box::new(StringLiteralRule),
        Box::new(NameRule),

        Box::new(TypeSeparatorRule),
        Box::new(RangeSeparatorRule),

        Box::new(OpenParamRule),
        Box::new(CloseParamRule),
        Box::new(OpenBracketRule),
        Box::new(CloseBracketRule),

    ]
}