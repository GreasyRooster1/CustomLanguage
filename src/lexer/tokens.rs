use crate::lexer::{Token, TokenRule};


struct FuncRule();
struct LoopRule();

struct NumberLiteralRule();
struct StringLiteralRule();
struct TypeNameRule();
struct NameRule();

struct TypeSeparatorRule();
struct RangeSeparatorRule();

struct OpenParamRule();
struct CloseParamRule();
struct OpenBracketRule();
struct CloseBracketRule();

impl TokenRule for FuncRule {
    fn check(string: String) -> bool {
        string == "#"
    }

    fn get_token(string: String) -> Token {
        Token::Func
    }
}

impl TokenRule for LoopRule {
    fn check(string: String) -> bool {
        string == "@"
    }

    fn get_token(string: String) -> Token {
        Token::Loop
    }
}

impl TokenRule for TypeSeparatorRule {
    fn check(string: String) -> bool {
        string == ":"
    }

    fn get_token(string: String) -> Token {
        Token::TypeSeparator
    }
}

impl TokenRule for RangeSeparatorRule {
    fn check(string: String) -> bool {
        string == ".."
    }

    fn get_token(string: String) -> Token {
        Token::RangeSeparator
    }
}

impl TokenRule for OpenParamRule {
    fn check(string: String) -> bool {
        string == "("
    }

    fn get_token(string: String) -> Token {
        Token::OpenParam
    }
}

impl TokenRule for CloseParamRule {
    fn check(string: String) -> bool {
        string == ")"
    }

    fn get_token(string: String) -> Token {
        Token::CloseParam
    }
}

impl TokenRule for OpenBracketRule {
    fn check(string: String) -> bool {
        string == "{"
    }

    fn get_token(string: String) -> Token {
        Token::OpenBracket
    }
}


impl TokenRule for NameRule {
    fn check(string: String) -> bool {
        string.starts_with(char::is_alphabetic) &&
        string.chars().all(char::is_alphanumeric)
    }

    fn get_token(string: String) -> Token {
        Token::Name(string)
    }
}

impl TokenRule for StringLiteralRule {
    fn check(string: String) -> bool {
        string.starts_with("\"") &&
        string.ends_with("\"")
    }

    fn get_token(string: String) -> Token {
        Token::Name(string)
    }
}

impl TokenRule for NumberLiteralRule {
    fn check(string: String) -> bool {
        string.parse::<f64>().is_ok() ||
        string.parse::<i32>().is_ok() ||
        string.parse::<u32>().is_ok()
    }

    fn get_token(string: String) -> Token {
        Token::Name(string)
    }
}
