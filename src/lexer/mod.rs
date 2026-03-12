mod lexer;
mod tests;
mod tokens;

const FUNC_LITERAL: &str = "#";
const LOOP_LITERAL: &str = "@";
const TYPE_SEPERATOR_LITERAL: &str = ":";
const RANGE_SEPERATOR_LITERAL: &str = "..";
const OPEN_PARAM_LITERAL: &str = "(";
const CLOSE_PARAM_LITERAL: &str = ")";
const OPEN_BRACKET_LITERAL: &str = "{";
const CLOSE_BRACKET_LITERAL: &str = "}";

#[derive(Debug)]
enum Token {
    // Keywords
    Func, // #
    Loop, // @

    // Literals
    NumberLiteral(String, NumberLiteralAssumptions),
    StringLiteral(String),
    //TypeName(String),
    Name(String),

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ..

    OpenParam,    // (
    CloseParam,   // )
    OpenBracket,  // {
    CloseBracket, // }
}

#[derive(Debug)]
enum NumberLiteralAssumptions {
    Float,
    Int,
    ExplicitRequired,
}

trait TokenRule {
    fn check(&self, string: &String) -> bool;

    fn get_token(&self, string: &String) -> Token;
}

