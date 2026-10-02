use std::ops::Deref;
use crate::lexer::Token::{Comma, OpenParam, TypeSeparator};
use crate::lexer::tokens::*;
use crate::lexer::{TokenRule, Token};
use log::{info, set_logger, warn};

pub(crate) fn get_matching_tokens(
    string: String,
    rules: &Vec<Box<dyn TokenRule>>,
) -> Vec<Token> {
    let mut tokens = vec![];

    for rule in rules {
        if rule.check(&string) {
            tokens.push(rule.get_token(&string))
        }
    }

    tokens
}

pub fn parse(text: String) -> Vec<Token>{
    let rules = alloc_rules_with_precidence();
    let mut output_tokens:Vec<Token> = vec![];
    let mut i = 0;
    let mut selector_length = text.len();
    while i < text.len() {
        if selector_length<1{
            i+=1;
            selector_length = text.len()-i;
        }
        let current_section = text.get(i..(i + selector_length)).expect("no section").to_string();
        let mut tokens = get_matching_tokens(current_section.clone(), &rules);
        if tokens.len() == 0 {
            selector_length -= 1;
            continue;
        }
        if tokens.len()>1 { println!("multiple matching tokens: {:?}", tokens); }

        output_tokens.push(tokens.get(0).expect("no token").clone());
        i+=selector_length;
        selector_length = text.len()-i;
    }

    output_tokens.push(Token::EOF);
    output_tokens
}

pub(crate) fn alloc_rules_with_precidence() -> Vec<Box<dyn TokenRule>> {
    vec![
        Box::new(FuncRule),
        Box::new(LoopRule),
        Box::new(IfRule),
        Box::new(TypeSeparatorRule),
        Box::new(RangeSeparatorRule),
        Box::new(StepSeparatorRule),
        Box::new(OpenParamRule),
        Box::new(CloseParamRule),
        Box::new(OpenBracketRule),
        Box::new(CloseBracketRule),
        Box::new(CommaRule),
        Box::new(AddRule),
        Box::new(SubRule),
        Box::new(MultRule),
        Box::new(DivRule),
        Box::new(ModRule),
        Box::new(ReturnRule),

        Box::new(NumberLiteralRule),
        Box::new(StringLiteralRule),
        Box::new(NameRule),
    ]
}

