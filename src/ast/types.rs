use crate::ast::Name;

enum Type{
    Int,
    Float,
    Bool,
    String,
    Function,
    ShortCircuit,
    Array(Box<Type>),
    Custom(CustomType),
}

struct CustomType{
    name:Name,

}