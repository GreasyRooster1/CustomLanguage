use crate::lexer::TokenType::{OpenParam, TypeSeparator};
use crate::lexer::tokens::*;
use crate::lexer::{TokenRule, TokenType};

pub(crate) fn get_matching_tokens(
    string: String,
    rules: &Vec<Box<dyn TokenRule>>,
) -> Vec<TokenType> {
    let mut tokens = vec![];

    for rule in rules {
        if rule.check(&string) {
            tokens.push(rule.get_token(&string))
        }
    }

    tokens
}

pub(crate) fn parse(text: String) -> Vec<TokenType>{
    let rules = alloc_rules();
    let mut output_tokens = vec![];
    let mut i = 0;
    let mut selector_length = 1;
    while i<text.len() {
        let currentSection = text.get(i..i + selector_length);
        let tokens = get_matching_tokens(text, &rules);
        if tokens.len() == 0 {
            selector_length += 1;
            continue;
        }
        if tokens.len()>1 { warn!("multiple matching tokens"); }

        output_tokens.append(tokens.get(0));
    }

    output_tokens
}

pub(crate) fn alloc_rules() -> Vec<Box<dyn TokenRule>> {
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

