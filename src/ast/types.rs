use crate::ast::Name;

enum Type{
    Int,
    Float,
    Bool,
    String,
    Function,
    ShortCircuit,
    Array(Box<Type>),
    Struct(Box<StructType>),
    ShadowType(Box<ShadowType>)
}

struct StructType{
    name:Name,
    fields:Vec<Type>
}
struct ShadowType{
    name:Name,
    shadows:Type,
}