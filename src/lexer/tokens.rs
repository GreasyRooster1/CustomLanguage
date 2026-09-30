use crate::lexer::Token::NumberLiteral;
use crate::lexer::{CLOSE_BRACKET_LITERAL, CLOSE_PARAM_LITERAL, FUNC_LITERAL, LOOP_LITERAL, NumberLiteralAssumption, OPEN_BRACKET_LITERAL, OPEN_PARAM_LITERAL, RANGE_SEPERATOR_LITERAL, TYPE_SEPERATOR_LITERAL, TokenRule, Token, NAME_ALLOWED_CHARS, TYPE_DENOTER_LITERAL, COMMA_LITERAL};

pub struct FuncRule;
pub struct LoopRule;

pub struct NumberLiteralRule;
pub struct StringLiteralRule;
// pub struct TypeNameRule;
pub struct NameRule;

pub struct TypeSeparatorRule;
pub struct RangeSeparatorRule;

pub struct OpenParamRule;
pub struct CloseParamRule;
pub struct OpenBracketRule;
pub struct CloseBracketRule;
pub struct CommaRule;


pub struct AddRule;
pub struct SubRule;
pub struct MultRule;
pub struct DivRule;
pub struct ModRule;

impl TokenRule for FuncRule {
    fn check(&self, string: &String) -> bool {
        string == FUNC_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Func
    }
}

impl TokenRule for LoopRule {
    fn check(&self, string: &String) -> bool {
        string == LOOP_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Loop
    }
}

impl TokenRule for TypeSeparatorRule {
    fn check(&self, string: &String) -> bool {
        string == TYPE_SEPERATOR_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::TypeSeparator
    }
}

impl TokenRule for RangeSeparatorRule {
    fn check(&self, string: &String) -> bool {
        string == RANGE_SEPERATOR_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::RangeSeparator
    }
}

impl TokenRule for OpenParamRule {
    fn check(&self, string: &String) -> bool {
        string == OPEN_PARAM_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::OpenParam
    }
}

impl TokenRule for CloseParamRule {
    fn check(&self, string: &String) -> bool {
        string == CLOSE_PARAM_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::CloseParam
    }
}

impl TokenRule for OpenBracketRule {
    fn check(&self, string: &String) -> bool {
        string == OPEN_BRACKET_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::OpenBracket
    }
}

impl TokenRule for CloseBracketRule {
    fn check(&self, string: &String) -> bool {
        string == CLOSE_BRACKET_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::CloseBracket
    }
}

impl TokenRule for CommaRule {
    fn check(&self, string: &String) -> bool {
        string == COMMA_LITERAL
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Comma
    }
}

impl TokenRule for AddRule {
    fn check(&self, string: &String) -> bool {
        string == "+"
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Add
    }
}
impl TokenRule for SubRule {
    fn check(&self, string: &String) -> bool {
        string == "-"
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Sub
    }
}
impl TokenRule for MultRule {
    fn check(&self, string: &String) -> bool {
        string == "*"
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Mult
    }
}
impl TokenRule for DivRule {
    fn check(&self, string: &String) -> bool {
        string == "/"
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Div
    }
}
impl TokenRule for ModRule {
    fn check(&self, string: &String) -> bool {
        string == "%"
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Mod
    }
}

impl TokenRule for NameRule {
    fn check(&self, string: &String) -> bool {
        string.starts_with(char::is_alphabetic) && string.chars().all(|c| NAME_ALLOWED_CHARS.contains(c))
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Name((*(string.clone())).parse().unwrap())
    }
}

impl TokenRule for StringLiteralRule {
    fn check(&self, string: &String) -> bool {
        string.starts_with("\"") && string.ends_with("\"")
    }

    fn get_token(&self, string: &String) -> Token {
        Token::Name((*(string.clone())).parse().unwrap())
    }
}

impl TokenRule for NumberLiteralRule {
    fn check(&self, string: &String) -> bool {
        string.parse::<f64>().is_ok()
            || string.parse::<i128>().is_ok()
    }

    fn get_token(&self, string: &String) -> Token {
        let mut assumption;
        let f32_val = string.parse::<f64>();
        let i128_val = string.parse::<i128>();
        if i128_val.is_ok() {
            assumption = NumberLiteralAssumption::Int(i128_val.expect("somehow failed"));
        } else if f32_val.is_ok() {
            assumption = NumberLiteralAssumption::Float(f32_val.expect("somehow failed"));
        }else{
            panic!("somehow failed as a number literal");
        }
        Token::NumberLiteral(assumption)
    }
}
