use std::collections::HashMap;
use std::sync::Arc;
use crate::ast::{Name, Parameter};
use crate::interpreter::functions::{DebugFunctionExecution, InsulateFunctionExecution};
use crate::interpreter::StackScope;
use crate::interpreter::value::{FunctionValue, Value, ValueType};

pub(crate) fn get_standard_scope() -> StackScope {
    let mut scope:HashMap<Name, Value> = HashMap::new();
    let mut types:HashMap<Name, ValueType> = HashMap::new();
    add_debug_functions(&mut scope);
    add_short_insulate(&mut scope);
    add_builtin_types(&mut types);
    StackScope {
        variables: scope,
        types
    }
}

fn add_debug_functions(scope:&mut HashMap<Name,Value>){
    scope.insert(Name("debugs".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(DebugFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("str".to_string()),
        }],
        return_value: Name("none".to_string()),
    }));

    scope.insert(Name("debugi".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(DebugFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("i".to_string()),
        }],
        return_value: Name("none".to_string()),
    }));

    scope.insert(Name("debugf".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(DebugFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("f".to_string()),
        }],
        return_value: Name("none".to_string()),
    }));

    scope.insert(Name("debugb".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(DebugFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("b".to_string()),
        }],
        return_value: Name("none".to_string()),
    }));

    scope.insert(Name("debugfunc".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(DebugFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("func".to_string()),
        }],
        return_value: Name("none".to_string()),
    }));

    scope.insert(Name("debugshort".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(DebugFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("shortc".to_string()),
        }],
        return_value: Name("none".to_string()),
    }));
}

fn add_short_insulate(scope:&mut HashMap<Name,Value>){
    scope.insert(Name("insulate".to_string()),Value::Function(FunctionValue{
        executor: Arc::new(InsulateFunctionExecution),
        parameters: vec![Parameter{
            name: Name("arg".to_string()),
            p_type: Name("shortc".to_string()),
        }],
        return_value: Name("any".to_string()),
    }));
}

fn add_builtin_types(scope:&mut HashMap<Name,ValueType>){
    scope.insert(Name("f".to_string()),ValueType::Float);
    scope.insert(Name("i".to_string()),ValueType::Int);
    scope.insert(Name("str".to_string()),ValueType::String);
    scope.insert(Name("b".to_string()),ValueType::Bool);
    scope.insert(Name("func".to_string()),ValueType::Function);
    scope.insert(Name("shortc".to_string()),ValueType::ShortCircuit);
}