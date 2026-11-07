use crate::ast::Name;

enum Type{
    Int,
    Float,
    Bool,
    String,
    Function,
    ShortCircuit,
    Array(Box<Type>),
    Struct(Box<Vec<Type>>),
    ShadowType(Box<Type>)
}
