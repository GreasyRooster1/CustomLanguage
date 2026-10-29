use serde::Serialize;
use strum::Display;

pub(crate) mod lexer;
mod tests;
mod tokens;

const FUNC_LITERAL: &str = "fn ";
const LOOP_LITERAL: &str = "loop";
const IF_LITERAL: &str = "if";
const TYPE_SEPERATOR_LITERAL: &str = ":";
const RANGE_SEPERATOR_LITERAL: &str = "->";
const OPEN_PARAM_LITERAL: &str = "(";
const CLOSE_PARAM_LITERAL: &str = ")";
const OPEN_BRACKET_LITERAL: &str = "{";
const CLOSE_BRACKET_LITERAL: &str = "}";
const TYPE_DENOTER_LITERAL: &str = "#";
const RETURN_LITERAL: &str = "ret";
const SHORT_LITERAL: &str = "short";
const COMMA_LITERAL: &str = ",";
const STEP_SEPERATOR_LITERAL: &str = "by";
const ASSIGNMENT_EQ_LITERAL: &str = "=";
const CHECK_EQ_LITERAL: &str = "==";
const CHECK_NOT_EQ_LITERAL: &str = "!=";
const GREATER_THAN_LITERAL: &str = ">";
const LESS_THAN_LITERAL: &str = "<";
const GREATER_THAN_EQ_LITERAL: &str = ">=";
const LESS_THAN_EQ_LITERAL: &str = "<=";
const AND_LITERAL: &str = "&&";
const OR_LITERAL: &str = "||";
const NOT_LITERAL: &str = "!";
const STATEMENT_SEPERATOR_LITERAL: &str = ";";

const NAME_ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

#[derive(Debug, Clone, Serialize,Display)]
pub enum Token {
    // Keywords
    Func, // # (fn)
    Loop, // @ (loop)
    If, // $ (if)

    // Literals
    NumberLiteral(NumberLiteralAssumption),
    StringLiteral(String),
    // TypeName(String),
    Name(String),

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ->
    StepSeparator, // by
    StatementSeparator, // ;

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
    
    AssignmentEq,
    CheckEq,
    CheckNotEq,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    And,
    Or,
    Not,
    
    Return,
    Short,
    EOF,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Keywords
    Func, // # (fn)
    Loop, // @ (loop)
    If, // $ (if)

    // Literals
    NumberLiteral,
    StringLiteral,
    Name,

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ->
    StepSeparator, // by
    StatementSeparator, // ;

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

    AssignmentEq,
    CheckEq,
    CheckNotEq,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    And,
    Or,
    Not,

    Return,
    Short,
    EOF,
}

impl PartialEq<&Token> for TokenType {
    fn eq(&self, other: &&Token) -> bool {
        match other {
            Token::Func => {matches!(self,TokenType::Func)}
            Token::Loop => {matches!(self,TokenType::Loop)}
            Token::If => {matches!(self,TokenType::If)}
            Token::NumberLiteral(_) => {matches!(self,TokenType::NumberLiteral)}
            Token::StringLiteral(_) => {matches!(self,TokenType::StringLiteral)}
            Token::Name(_) => {matches!(self,TokenType::Name)}
            Token::TypeSeparator => {matches!(self,TokenType::TypeSeparator)}
            Token::RangeSeparator => {matches!(self,TokenType::RangeSeparator)}
            Token::StepSeparator => {matches!(self,TokenType::StepSeparator)}
            Token::StatementSeparator => {matches!(self,TokenType::StatementSeparator)}
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
            Token::AssignmentEq => {matches!(self,TokenType::AssignmentEq)}
            Token::CheckEq => {matches!(self,TokenType::CheckEq)}
            Token::CheckNotEq => {matches!(self,TokenType::CheckNotEq)}
            Token::GreaterThan => {matches!(self,TokenType::GreaterThan)}
            Token::LessThan => {matches!(self,TokenType::LessThan)}
            Token::GreaterThanEq => {matches!(self,TokenType::GreaterThanEq)}
            Token::LessThanEq => {matches!(self,TokenType::LessThanEq)}
            Token::And => {matches!(self,TokenType::And)}
            Token::Or => {matches!(self,TokenType::Or)}
            Token::Not => {matches!(self,TokenType::Not)}
            Token::Return => {matches!(self,TokenType::Return)},
            Token::Short => {matches!(self,TokenType::Short)},
            Token::EOF => {matches!(self,TokenType::EOF)}
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub enum NumberLiteralAssumption {
    Float(f32),
    Int(i32)
}

trait TokenRule {
    fn check(&self, string: &String) -> bool;

    fn get_token(&self, string: &String) -> Token;
}

