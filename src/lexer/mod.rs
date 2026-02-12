mod tokens;
mod lexer;

enum Token{
    // Keywords
    Func,  // #
    Loop,  // @

    // Literals
    NumberLiteral(String),
    StringLiteral(String),
    TypeName(String),
    Name(String),

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ..

    OpenParam, // (
    CloseParam, // )
    OpenBracket, // {
    CloseBracket, // }
}

trait TokenRule{
    fn check(string: String) -> bool;

    fn get_token(string: String) -> Token;
}