use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::Matches;
use std::sync::Arc;
use serde::Serialize;
use strum::Display;
use crate::ast::{Name, Parameter};
use crate::error::{CompileError, CompileResult};
use crate::interpreter::functions::FunctionExecution;

#[derive(Debug,Clone,Serialize,Display, PartialEq)]
pub(crate) enum Value{
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    #[serde(skip_serializing)]
    Function(FunctionValue),
    NoValue,
    ShortCircuit(Box<Value>),

    Array(Vec<Value>),
    Struct(Box<HashMap<Name,Value>>),
    Type(Box<ValueType>)
}

#[derive(Debug,Clone,Serialize,Display, PartialEq)]
pub(crate) enum ValueType{
    Float,
    Int,
    String,
    Bool,
    Function,
    NoValue,
    ShortCircuit,

    Struct(StructType),
    Array(ArrayType),
    Type
}

#[derive(Clone, Debug, PartialEq,Serialize)]
pub(crate) struct StructType{
    fields: HashMap<Name,ValueType>,
}
#[derive(Clone, Debug, PartialEq,Serialize)]
pub(crate) struct ArrayType{
    data: Vec<Value>,
    data_type:Box<ValueType>,
}

#[derive(Clone, Debug)]
pub struct FunctionValue{
    pub(crate) executor:Arc<dyn FunctionExecution>,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) return_value: Name,
}

impl Value{
    fn is_num(&self) -> bool{
        *self==ValueType::Float || *self==ValueType::Int
    }
}

impl Add for Value{
    type Output = CompileResult<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        if !(self.is_num() || rhs.is_num()){
            return Err(CompileError::NotNumber);
        }
        if self==ValueType::Int && rhs==ValueType::Int{
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a+b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a+b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32+b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a+b))
        }else{
            Err(CompileError::Unspecified)
        }

    }
}

impl Sub for Value{
    type Output = CompileResult<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        if !(self.is_num() || rhs.is_num()){
            return Err(CompileError::NotNumber);
        }
        if self==ValueType::Int && rhs==ValueType::Int{
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a-b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a-b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32-b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a-b))
        }else{
            Err(CompileError::Unspecified)
        }

    }
}

impl Mul for Value{
    type Output = CompileResult<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        if !(self.is_num() || rhs.is_num()){
            return Err(CompileError::NotNumber);
        }
        if self==ValueType::Int && rhs==ValueType::Int{
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a*b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a*b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32*b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a*b))
        }else{
            Err(CompileError::Unspecified)
        }

    }
}

impl Div for Value{
    type Output = CompileResult<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        if !(self.is_num() || rhs.is_num()){
            return Err(CompileError::NotNumber);
        }
        if self==ValueType::Int && rhs==ValueType::Int{
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a/b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a/b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32/b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a/b))
        }else{
            Err(CompileError::Unspecified)
        }

    }
}

impl Rem for Value{
    type Output = CompileResult<Value>;

    fn rem(self, rhs: Self) -> Self::Output {
        if !(self.is_num() || rhs.is_num()){
            return Err(CompileError::NotNumber);
        }
        if self==ValueType::Int && rhs==ValueType::Int{
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a%b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a%b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32%b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =rhs else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a%b))
        }else{
            Err(CompileError::Unspecified)
        }

    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if !(self.is_num() || rhs.is_num()){
            return None;
        }
        if *self==ValueType::Int && *rhs==ValueType::Int{
            let Value::Int(a) =self else { return None };
            let Value::Int(b) =rhs else { return None };
            PartialOrd::partial_cmp(a, b)
        }else if *self==ValueType::Float && *rhs==ValueType::Int{
            let Value::Float(a) =self else { return None };
            let Value::Int(b) =rhs else { return None };
            PartialOrd::partial_cmp(a, &(*b as f32))
        }else if *self==ValueType::Int && *rhs==ValueType::Float {
            let Value::Int(a) =self else { return None };
            let Value::Float(b) =rhs else { return None };
            PartialOrd::partial_cmp(&(*a as f32), b)
        }else if *self==ValueType::Float && *rhs==ValueType::Float {
            let Value::Float(a) =self else { return None };
            let Value::Float(b) =rhs else { return None };
            PartialOrd::partial_cmp(a, b)
        }else{
            None
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
            "shortc" => ValueType::ShortCircuit,
            _ => ValueType::NoValue
        }
    }
}

impl PartialEq<ValueType> for Value {
    fn eq(&self, other: &ValueType) -> bool {
        match self{
            Value::Float(_) => matches!(other,ValueType::Float),
            Value::Int(_) => matches!(other,ValueType::Float),
            Value::String(_) => matches!(other,ValueType::Float),
            Value::Bool(_) => matches!(other,ValueType::Bool),
            Value::Function(_) => matches!(other,ValueType::Function),
            Value::NoValue => matches!(other,ValueType::NoValue),
            Value::ShortCircuit(_) => matches!(other,ValueType::ShortCircuit),
            Value::Array(array) => {
                for val in array{
                    if val!=other{
                        return false;
                    }
                }
                true
            }
            Value::Struct(st) => {
                let ValueType::Struct(other_struct) = other else {return false};
                let str = *(st.clone());
                for (key,value) in str{
                    if value!=other_struct.fields.get(&key).unwrap() {
                        return false;
                    }
                }
                true
            }
            Value::Type(_)=>matches!(other,ValueType::Type)
        }
    }
}

impl PartialEq<Value> for ValueType {
    fn eq(&self, other: &Value) -> bool {
        other == self
    }
}

impl PartialEq<&ValueType> for Value {
    fn eq(&self, other: &&ValueType) -> bool {
        **other == *self
    }
}