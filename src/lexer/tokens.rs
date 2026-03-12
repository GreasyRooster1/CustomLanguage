use crate::lexer::TokenType::NumberLiteral;
use crate::lexer::{
    CLOSE_BRACKET_LITERAL, CLOSE_PARAM_LITERAL, FUNC_LITERAL, LOOP_LITERAL,
    NumberLiteralAssumptions, OPEN_BRACKET_LITERAL, OPEN_PARAM_LITERAL, RANGE_SEPERATOR_LITERAL,
    TYPE_SEPERATOR_LITERAL, TokenRule, TokenType,
};

pub struct FuncRule;
pub struct LoopRule;

pub struct NumberLiteralRule;
pub struct StringLiteralRule;
pub struct TypeNameRule;
pub struct NameRule;

pub struct TypeSeparatorRule;
pub struct RangeSeparatorRule;

pub struct OpenParamRule;
pub struct CloseParamRule;
pub struct OpenBracketRule;
pub struct CloseBracketRule;

impl TokenRule for FuncRule {
    fn check(&self, string: &String) -> bool {
        string == FUNC_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::Func
    }
}

impl TokenRule for LoopRule {
    fn check(&self, string: &String) -> bool {
        string == LOOP_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::Loop
    }
}

impl TokenRule for TypeSeparatorRule {
    fn check(&self, string: &String) -> bool {
        string == TYPE_SEPERATOR_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::TypeSeparator
    }
}

impl TokenRule for RangeSeparatorRule {
    fn check(&self, string: &String) -> bool {
        string == RANGE_SEPERATOR_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::RangeSeparator
    }
}

impl TokenRule for OpenParamRule {
    fn check(&self, string: &String) -> bool {
        string == OPEN_PARAM_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::OpenParam
    }
}

impl TokenRule for CloseParamRule {
    fn check(&self, string: &String) -> bool {
        string == CLOSE_PARAM_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::CloseParam
    }
}

impl TokenRule for OpenBracketRule {
    fn check(&self, string: &String) -> bool {
        string == OPEN_BRACKET_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::OpenBracket
    }
}

impl TokenRule for CloseBracketRule {
    fn check(&self, string: &String) -> bool {
        string == CLOSE_BRACKET_LITERAL
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::CloseBracket
    }
}

impl TokenRule for NameRule {
    fn check(&self, string: &String) -> bool {
        string.starts_with(char::is_alphabetic) && string.chars().all(char::is_alphanumeric)
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::Name((*(string.clone())).parse().unwrap())
    }
}

impl TokenRule for StringLiteralRule {
    fn check(&self, string: &String) -> bool {
        string.starts_with("\"") && string.ends_with("\"")
    }

    fn get_token(&self, string: &String) -> TokenType {
        TokenType::Name((*(string.clone())).parse().unwrap())
    }
}

impl TokenRule for NumberLiteralRule {
    fn check(&self, string: &String) -> bool {
        string.parse::<f64>().is_ok()
            || string.parse::<i128>().is_ok()
            || string.parse::<u128>().is_ok()
    }

    fn get_token(&self, string: &String) -> TokenType {
        let mut assumption;
        let value = string.parse::<f64>();
        if string.parse::<i32>().is_ok() {
            assumption = NumberLiteralAssumptions::Int;
        } else if string.parse::<f32>().is_ok() {
            assumption = NumberLiteralAssumptions::Float
        } else {
            assumption = NumberLiteralAssumptions::ExplicitRequired
        }
        TokenType::NumberLiteral((*(string.clone())).parse().unwrap(), assumption)
    }
}
