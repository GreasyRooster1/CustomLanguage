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

impl TokenRule for CloseBracketRule {
    fn check(&self, string: String) -> bool {
        string == "}"
    }

    fn get_token(&self, string: String) -> Token {
        Token::CloseBracket
    }
}
