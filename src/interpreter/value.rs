use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Rem, Sub};
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

impl Value{
    fn is_num(&self) -> bool{
        *self==ValueType::Float || *self==ValueType::Int
    }
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

impl Add for Value{
    type Output = CompileResult<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        if !(self.is_num() || rhs.is_num()){
            return Err(CompileError::NotNumber);
        }
        if self==ValueType::Int && rhs==ValueType::Int{
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a+b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a+b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32+b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
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
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a-b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a-b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32-b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
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
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a*b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a*b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32*b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
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
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a/b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a/b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32/b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
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
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Int(a%b))
        }else if(self==ValueType::Float && rhs==ValueType::Int){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a%b as f32))
        }else if(self==ValueType::Int && rhs==ValueType::Float){
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
            Ok(Value::Float(a as f32%b))
        }else if(self==ValueType::Float && rhs==ValueType::Float){
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
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
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            PartialOrd::partial_cmp(a, b)
        }else if *self==ValueType::Float && *rhs==ValueType::Int{
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Int(b) =self else { Err(CompileError::Unspecified)? };
            PartialOrd::partial_cmp(a, &(*b as f32))
        }else if *self==ValueType::Int && *rhs==ValueType::Float {
            let Value::Int(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
            PartialOrd::partial_cmp(&(*a as f32), b)
        }else if *self==ValueType::Float && *rhs==ValueType::Float {
            let Value::Float(a) =self else { Err(CompileError::Unspecified)? };
            let Value::Float(b) =self else { Err(CompileError::Unspecified)? };
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