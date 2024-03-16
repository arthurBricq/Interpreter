use std::collections::HashMap;
use crate::ast::expression::{Expr, Value};
use crate::error::EvalError;

/// Standard Library
pub struct Std;

const PRINT: &'static str = "print";

impl Std {
    pub fn is_in_standard_lib(name: &String) -> bool {
        if name.as_str() == PRINT {
            return true    
        }
        false
    }
    
    pub fn eval(name: &String, args: &Vec<Value>) -> Result<Value, EvalError> {
        match name.as_str() {
            PRINT => Self::print(args),
            _ => {}
        }
        Ok(Value::None)
    }
    
    fn print(args: &Vec<Value>) {
        for value in args {
            println!("{value}")
        }
    }
    
}
    