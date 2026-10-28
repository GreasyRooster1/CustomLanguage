use custom_error::custom_error;
use crate::interpreter::Value;

custom_error! {
    pub CompileError
    Unspecified = "unspecified",
    WrongTypeAssumption{expected:Value} = "wrong type, expected type '{expected}'",
}