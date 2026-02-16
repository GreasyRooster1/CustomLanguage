mod tokens;
mod lexer;
mod tests;

#[derive(Debug)]
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

#[derive(Debug)]
enum NumberLiteralAssumptions{
    Float,
    Int,
    ExplicitRequired,
}

trait TokenRule{
    fn check(&self,string: &String) -> bool;

    fn get_token(&self,string: &String) -> Token;
}