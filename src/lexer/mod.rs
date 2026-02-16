mod tokens;
mod lexer;

enum Token{
    // Keywords
    Func,  // #
    Loop,  // @

    // Literals
    NumberLiteral(String,BuiltinNumberType),
    StringLiteral(String),
    //TypeName(String),
    Name(String),

    // Characters
    TypeSeparator,  // :
    RangeSeparator, // ..

    OpenParam, // (
    CloseParam, // )
    OpenBracket, // {
    CloseBracket, // }
}

enum NumberLiteralAssumptions{
    Float,
    Int,
    UInt,
}

trait TokenRule{
    fn check(string: String) -> bool;

    fn get_token(string: String) -> Token;
}