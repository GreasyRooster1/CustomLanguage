use crate::lexer::{NumberLiteralAssumptions, Token, TokenRule};
use crate::lexer::Token::NumberLiteral;

struct FuncRule;
struct LoopRule;

struct NumberLiteralRule;
struct StringLiteralRule;
struct TypeNameRule;
struct NameRule;

struct TypeSeparatorRule;
struct RangeSeparatorRule;

struct OpenParamRule;
struct CloseParamRule;
struct OpenBracketRule;
struct CloseBracketRule;

impl TokenRule for FuncRule {
    fn check(&self, string: String) -> bool {
        string == "#"
    }

    fn get_token(&self, string: String) -> Token {
        Token::Func
    }
}

impl TokenRule for LoopRule {
    fn check(&self, string: String) -> bool {
        string == "@"
    }

    fn get_token(&self, string: String) -> Token {
        Token::Loop
    }
}

impl TokenRule for TypeSeparatorRule {
    fn check(&self, string: String) -> bool {
        string == ":"
    }

    fn get_token(&self, string: String) -> Token {
        Token::TypeSeparator
    }
}

impl TokenRule for RangeSeparatorRule {
    fn check(&self, string: String) -> bool {
        string == ".."
    }

    fn get_token(&self, string: String) -> Token {
        Token::RangeSeparator
    }
}

impl TokenRule for OpenParamRule {
    fn check(&self, string: String) -> bool {
        string == "("
    }

    fn get_token(&self, string: String) -> Token {
        Token::OpenParam
    }
}

impl TokenRule for CloseParamRule {
    fn check(&self, string: String) -> bool {
        string == ")"
    }

    fn get_token(&self, string: String) -> Token {
        Token::CloseParam
    }
}

impl TokenRule for OpenBracketRule {
    fn check(&self, string: String) -> bool {
        string == "{"
    }

    fn get_token(&self, string: String) -> Token {
        Token::OpenBracket
    }
}


impl TokenRule for NameRule {
    fn check(&self, string: String) -> bool {
        string.starts_with(char::is_alphabetic) &&
        string.chars().all(char::is_alphanumeric)
    }

    fn get_token(&self, string: String) -> Token {
        Token::Name(string)
    }
}

impl TokenRule for StringLiteralRule {
    fn check(&self, string: String) -> bool {
        string.starts_with("\"") &&
        string.ends_with("\"")
    }

    fn get_token(&self, string: String) -> Token {
        Token::Name(string)
    }
}

impl TokenRule for NumberLiteralRule {
    fn check(&self, string: String) -> bool {
        string.parse::<f64>().is_ok() ||
        string.parse::<i128>().is_ok() ||
        string.parse::<u128>().is_ok()
    }

    fn get_token(&self, string: String) -> Token {
        let mut assumption;
        let value = string.parse::<f64>();
        if string.parse::<i32>().is_ok(){
            assumption = NumberLiteralAssumptions::Int;
        }else if string.parse::<f32>().is_ok(){
            assumption = NumberLiteralAssumptions::Float
        }else{
            assumption = NumberLiteralAssumptions::ExplicitRequired
        }
        Token::NumberLiteral(string,assumption)
    }
}
