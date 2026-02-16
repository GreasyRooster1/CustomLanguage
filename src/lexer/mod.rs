mod tokens;
mod lexer;

enum Token{
    // Keywords
    Func,  // #
    Loop,  // @

    // Literals
    NumberLiteral(String,NumberLiteralAssumptions),
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
    ExplicitRequired,
}

trait TokenRule{
    fn check(&self,string: String) -> bool;

    fn get_token(&self,string: String) -> Token;
}