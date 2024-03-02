use std::collections::HashMap;
use crate::ast::expression::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, IdentExpr, ParenthesisExpr};
use crate::error::EvalError;
use crate::error::EvalError::{MultipleError, UnknownVariable};
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
    pub fn eval(&self, buf: &mut HashMap<String, i64>) -> Result<i64, EvalError> {
        match self {
            ConstExpr(value) => Ok(*value),
            Expr::NegExpr(expr) => match expr.eval(buf) {
                Ok(value) => Ok(-value),
                Err(e) => Err(e),
            }
            ParenthesisExpr(expr) => expr.eval(buf),
            BinaryExpr(l, op, r) => match (l.eval(buf), r.eval(buf)) {
                (Ok(l), Ok(r)) => Ok(match op {
                    Op::Plus => l + r,
                    Op::Minus => l - r,
                    Op::Times => l * r,
                    Op::Div => l / r,
                }),
                (Err(r), Ok(_)) => Err(r),
                (Ok(_), Err(err)) => Err(err),
                (Err(err1), Err(err2)) => Err(MultipleError(vec![Box::new(err1), Box::new(err2)])),
            }
            AssignmentExpr(name, value) => {
                let eval = value.eval(buf);
                match eval {
                    Ok(value) => {
                        buf.insert(name.clone(), value);
                    }
                    _ => {}
                }
                eval
            }
            IdentExpr(name) => match buf.get(name) {
                Some(value) => Ok(*value),
                None => Err(UnknownVariable(name.clone())),
            }
            Expr::FunctionCall(_name, _args) => {
                panic!("Function calls are not supported")
            }
        }
    }
}
