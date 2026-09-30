

pub(crate) mod lexer;
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
const RETURN_LITERAL: &str = "ret";
const COMMA_LITERAL: &str = ",";

const NAME_ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_";

#[derive(Debug, Clone)]
pub enum Token {
    // Keywords
    Func, // # (fn)
    Loop, // @ (loop)

    // Literals
    NumberLiteral(NumberLiteralAssumption),
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
    Comma, // ,

    Add,
    Sub,
    Mult,
    Div,
    Mod,
    
    Return
}

#[derive(Debug, Clone)]
pub enum TokenType {
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
    Comma, // ,

    Add,
    Sub,
    Mult,
    Div,
    Mod,
    
    Return
}

impl PartialEq<&Token> for TokenType {
    fn eq(&self, other: &&Token) -> bool {
        match other {
            Token::Func => {matches!(self,TokenType::Func)}
            Token::Loop => {matches!(self,TokenType::Loop)}
            Token::NumberLiteral(_) => {matches!(self,TokenType::NumberLiteral)}
            Token::StringLiteral(_) => {matches!(self,TokenType::StringLiteral)}
            Token::Name(_) => {matches!(self,TokenType::Name)}
            Token::TypeSeparator => {matches!(self,TokenType::TypeSeparator)}
            Token::RangeSeparator => {matches!(self,TokenType::RangeSeparator)}
            Token::OpenParam => {matches!(self,TokenType::OpenParam)}
            Token::CloseParam => {matches!(self,TokenType::CloseParam)}
            Token::OpenBracket => {matches!(self,TokenType::OpenBracket)}
            Token::CloseBracket => {matches!(self,TokenType::CloseBracket)}
            Token::Comma => {matches!(self,TokenType::Comma)}
            Token::Add => {matches!(self,TokenType::Add)}
            Token::Sub => {matches!(self,TokenType::Sub)}
            Token::Mult => {matches!(self,TokenType::Mult)}
            Token::Div => {matches!(self,TokenType::Div)}
            Token::Mod => {matches!(self,TokenType::Mult)}
            Token::Return => {matches!(self,TokenType::Return)}
        }
    }
}


#[derive(Debug, Clone)]
pub enum NumberLiteralAssumption {
    Float(f64),
    Int(i128)
}

trait TokenRule {
    fn check(&self, string: &String) -> bool;

    fn get_token(&self, string: &String) -> Token;
}

