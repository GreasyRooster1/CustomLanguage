

mod lexer;
mod tests;
mod tokens;

const FUNC_LITERAL: &str = "fn ";
const LOOP_LITERAL: &str = "loop ";
const TYPE_SEPERATOR_LITERAL: &str = ":";
const RANGE_SEPERATOR_LITERAL: &str = "->";
const OPEN_PARAM_LITERAL: &str = "(";
const CLOSE_PARAM_LITERAL: &str = ")";
const OPEN_BRACKET_LITERAL: &str = "{";
const CLOSE_BRACKET_LITERAL: &str = "}";
const TYPE_DENOTER_LITERAL: &str = "#";

const NAME_ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_";

#[derive(Debug, Clone)]
pub(crate) enum Token {
    // Keywords
    Func, // # (fn)
    Loop, // @ (loop)

    // Literals
    NumberLiteral(String, NumberLiteralAssumption),
    StringLiteral(String),
    // TypeName(String),
    Name(String),

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ->

    OpenParam,    // (
    CloseParam,   // )
    OpenBracket,  // {
    CloseBracket, // }
}

#[derive(Debug, Clone)]
pub(crate) enum TokenType {
    // Keywords
    Func, // # (fn)
    Loop, // @ (loop)

    // Literals
    NumberLiteral,
    StringLiteral,
    Name,

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ->

    OpenParam,    // (
    CloseParam,   // )
    OpenBracket,  // {
    CloseBracket, // }
}

impl PartialEq<&Token> for TokenType {
    fn eq(&self, other: &&Token) -> bool {
        match other {
            Token::Func => {matches!(self,TokenType::Func)}
            Token::Loop => {matches!(self,TokenType::Loop)}
            Token::NumberLiteral(_, _) => {matches!(self,TokenType::NumberLiteral)}
            Token::StringLiteral(_) => {matches!(self,TokenType::StringLiteral)}
            Token::Name(_) => {matches!(self,TokenType::Name)}
            Token::TypeSeparator => {matches!(self,TokenType::TypeSeparator)}
            Token::RangeSeparator => {matches!(self,TokenType::RangeSeparator)}
            Token::OpenParam => {matches!(self,TokenType::OpenParam)}
            Token::CloseParam => {matches!(self,TokenType::CloseParam)}
            Token::OpenBracket => {matches!(self,TokenType::OpenBracket)}
            Token::CloseBracket => {matches!(self,TokenType::CloseBracket)}
        }
    }
}


#[derive(Debug, Clone)]
enum NumberLiteralAssumption {
    Float,
    Int,
    ExplicitRequired,
}

trait TokenRule {
    fn check(&self, string: &String) -> bool;

    fn get_token(&self, string: &String) -> Token;
}

