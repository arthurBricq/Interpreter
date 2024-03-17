use std::collections::HashMap;
use crate::ast::expression::{Expr, Value};
use crate::error::EvalError;

/// Standard Library
pub struct Std;

const PRINT: &'static str = "print";
const LEN: &'static str = "len";

impl Std {
    pub fn is_in_standard_lib(name: &String) -> bool {
        if let PRINT | LEN = name.as_str() {
            return true
        }
        false
    }

    pub fn eval(name: &String, args: &Vec<Value>) -> Result<Value, EvalError> {
        match name.as_str() {
            PRINT => Self::print(args),
            LEN => return Self::get_list_length(args),
            _ => {}
        }
        Ok(Value::None)
    }

    fn print(args: &Vec<Value>) {
        for value in args {
            println!("{value}")
        }
    }

    fn get_list_length(args: &Vec<Value>) -> Result<Value, EvalError> {
        if args.len() != 1 {
            Err(EvalError::Error("The function `len` can only be used with a single argument"))
        } else {
            match &args[0] {
                Value::List(data) =>  Ok(Value::IntValue(data.len() as i64)),
                _ => Err(EvalError::Error("The function `len` can only be used a value of type `list`"))
            }
        }
    }

}
