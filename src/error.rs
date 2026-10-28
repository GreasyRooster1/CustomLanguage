use custom_error::custom_error;
use crate::interpreter::value::ValueType;

custom_error! {
    pub CompileError
    Unspecified = "unspecified",
    
    WrongTypeAssumption{expected:ValueType} = "wrong type, expected type '{expected}'",
    ValueNotFound = "value not found",
    ArgsDontMatchSig = "args dont match signature",
    IfConditionNotBool="condition not bool",
    NotNumber = "not a number",
    InvalidOperation = "invalid operation",
}

pub type CompileResult<T> = std::result::Result<T, CompileError>;