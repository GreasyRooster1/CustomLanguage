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
