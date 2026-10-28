use std::sync::Arc;
use serde::Serialize;
use strum::Display;
use crate::ast::{Name, Parameter};
use crate::interpreter::functions::FunctionExecution;

#[derive(Debug,Clone,Serialize,Display, PartialEq)]
pub(crate) enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    #[serde(skip_serializing)]
    Function(FunctionValue),
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

#[derive(Clone, Debug)]
pub struct FunctionValue{
    pub(crate) executor:Arc<dyn FunctionExecution>,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) return_value: Name,
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

impl PartialEq for FunctionValue{
    fn eq(&self, other: &Self) -> bool {
        self.parameters==other.parameters && self.return_value==other.return_value
    }
}

impl From<Name> for ValueType{
    fn from(value: Name) -> Self {
        match value.0.as_str() {
            "f" => ValueType::Float,
            "i" => ValueType::Int,
            "str" => ValueType::String,
            "b" => ValueType::Bool,
            "func" => ValueType::Function,
            _ => ValueType::NoValue
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