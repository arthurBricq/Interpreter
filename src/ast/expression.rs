use std::collections::HashMap;
use crate::ast::declaration::Declaration;
use crate::ast::expression::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, FunctionCall, IdentExpr, ParenthesisExpr};
use crate::error::EvalError;
use crate::error::EvalError::{MultipleError, UnknownVariable};
use crate::module::Module;
use crate::token::Op;

/// An expression is something that evaluates to something
#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    ConstExpr(i64),
    NegExpr(Box<Expr>),
    ParenthesisExpr(Box<Expr>),
    BinaryExpr(Box<Expr>, Op, Box<Expr>),
    AssignmentExpr(String, Box<Expr>),
    IdentExpr(String),
    FunctionCall(String, Vec<Box<Expr>>),
}

impl Expr {
    /// Evaluates the expression
    /// buf: local variables (at the current scope)
    /// module: current evaluation module
    pub fn eval(&self, buf: &mut HashMap<String, i64>, module: Option<&Module>) -> Result<Option<i64>, EvalError> {
        match self {
            ConstExpr(value) => Ok(Some(*value)),
            Expr::NegExpr(expr) => match expr.eval(buf, module) {
                Ok(value) => Ok(Some(-value.unwrap())),
                Err(e) => Err(e),
            }
            ParenthesisExpr(expr) => expr.eval(buf, module),
            BinaryExpr(l, op, r) => match (l.eval(buf, module), r.eval(buf, module)) {
                (Ok(Some(l)), Ok(Some(r))) => Ok(Some(match op {
                    Op::Plus => l + r,
                    Op::Minus => l - r,
                    Op::Times => l * r,
                    Op::Div => l / r,
                })),
                (Err(r), Ok(_)) => Err(r),
                (Ok(_), Err(err)) => Err(err),
                (Err(err1), Err(err2)) => Err(MultipleError(vec![Box::new(err1), Box::new(err2)])),
                _ => panic!("Not sure what is happening")
            }
            AssignmentExpr(name, value) => {
                let eval = value.eval(buf, module);
                match eval {
                    Ok(Some(value)) => {
                        buf.insert(name.clone(), value);
                    }
                    _ => {}
                }
                eval
            }
            IdentExpr(name) => match buf.get(name) {
                Some(value) => Ok(Some(*value)),
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
                        if let Ok(Some(value)) = arg_expr.eval(buf, module) {
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
