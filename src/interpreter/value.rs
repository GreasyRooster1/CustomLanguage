use std::sync::Arc;
use serde::Serialize;
use strum::Display;
use crate::interpreter::FunctionExecution;

#[derive(Debug,Clone,Serialize,Display)]
pub(crate) enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    #[serde(skip_serializing)]
    Function(Arc<dyn FunctionExecution>),
    NoValue
}

#[derive(Debug,Clone,Serialize,Display, PartialEq)]
pub(crate) enum ValueType{
    Float,
    Int,
    String,
    Bool,
    Function,
    NoValue
}

impl From<&Value> for ValueType{
    fn from(value: &Value) -> Self {
        match value{
            Value::Float(_) => ValueType::Float,
            Value::Int(_) => ValueType::Int,
            Value::String(_) => ValueType::String,
            Value::Bool(_) => ValueType::Bool,
            Value::Function(_) => ValueType::Function,
            Value::NoValue => ValueType::NoValue,
        }
    }
}

impl PartialEq<ValueType> for Value {
    fn eq(&self, other: &ValueType) -> bool {
        let converted = ValueType::from(self);
        *other==converted
    }
}

// 2. Enables: SignalColor == PrimaryColor
impl PartialEq<Value> for ValueType {
    fn eq(&self, other: &Value) -> bool {
        other == self
    }
}