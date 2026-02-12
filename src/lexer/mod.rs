

struct Token{

}

enum Tokens{
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