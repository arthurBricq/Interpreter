use std::collections::HashMap;
use crate::ast::declaration::Declaration;
use crate::ast::expression::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, FunctionCall, IdentExpr, ParenthesisExpr};
use crate::ast::expression::Value::IntValue;
use crate::error::EvalError;
use crate::error::EvalError::{MultipleError, UnknownVariable};
use crate::module::Module;
use crate::token::Op;

/// A value is the result of an evaluation
/// It can be None, if there is no value
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    IntValue(i64),
    BoolValue(bool),
    None
}

/// An expression is something that evaluates to something
#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    ConstExpr(Value),
    NegExpr(Box<Expr>),
    ParenthesisExpr(Box<Expr>),
    BinaryExpr(Box<Expr>, Op, Box<Expr>),
    AssignmentExpr(String, Box<Expr>),
    IdentExpr(String),
    FunctionCall(String, Vec<Expr>),
}

impl Expr {
    /// Evaluates the expression
    /// buf: local variables (at the current scope)
    /// module: current evaluation module
    pub fn eval(&self, buf: &mut HashMap<String, Value>, module: Option<&Module>) -> Result<Value, EvalError> {
        match self {
            ConstExpr(value) => Ok(value.clone()),
            Expr::NegExpr(expr) => match expr.eval(buf, module) {
                Ok(IntValue(value)) => Ok(IntValue(-value)),
                Err(e) => Err(e),
                _ => Err(EvalError::Error("A negative express only applies to type Int and Float"))
            }
            ParenthesisExpr(expr) => expr.eval(buf, module),
            BinaryExpr(l, op, r) => match (l.eval(buf, module), r.eval(buf, module)) {
                (Ok(IntValue(l)), Ok(IntValue(r))) => Ok(IntValue(match op {
                    Op::Plus => l + r,
                    Op::Minus => l - r,
                    Op::Times => l * r,
                    Op::Div => l / r,
                })),
                (Err(r), Ok(_)) => Err(r),
                (Ok(_), Err(err)) => Err(err),
                (Err(err1), Err(err2)) => Err(MultipleError(vec![Box::new(err1), Box::new(err2)])),
                _ => panic!("Not sure what is happening... You will have to debug this :'(")
            }
            AssignmentExpr(name, value) => {
                let eval = value.eval(buf, module);
                match eval {
                    Ok(value) => buf.insert(name.clone(), value.clone()),
                    _ => None
                };
                Ok(Value::None)
            }
            IdentExpr(name) => match buf.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(UnknownVariable(name.clone())),
            }
            FunctionCall(name, inputs) => {
                if module.is_none() {
                    return Err(EvalError::Error("Module not found"))
                }
                if let Some(Declaration::Function(_name, args, func)) =  module.unwrap().get_function(name) {
                    // i. evaluate the inputs
                    let mut function_inputs = HashMap::new();
                    for i in 0..args.len() {
                        let arg_name = &args[i];
                        let arg_expr = &inputs[i];
                        if let Ok(value) = arg_expr.eval(buf, module) {
                            function_inputs.insert(arg_name.0.clone(), value);
                        }
                    }
                    // We don't provide the function call with all the variables, but just with the provided arguments
                    func.eval(&mut function_inputs, module)
                } else {
                    Err(EvalError::Error("Function not found"))
                }
            }
        }
    }
}
