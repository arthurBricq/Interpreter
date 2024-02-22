use std::collections::HashMap;

use crate::ast::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, IdentExpr, NegExpr, ParenthesisExpr};
use crate::ast::ParserError::{MultipleError, UnknownError, UnknownVariable};
use crate::parser::Parser;
use crate::token::{Op, Token};

/// An expression is something that evaluates to something
#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    ConstExpr(i64),
    NegExpr(Box<Expr>),
    ParenthesisExpr(Box<Expr>),
    BinaryExpr(Box<Expr>, Op, Box<Expr>),
    AssignmentExpr(String, Box<Expr>),
    IdentExpr(String),
}

#[derive(Debug)]
pub enum ParserError {
    UnknownError,
    UnknownVariable(String),
    MultipleError(Vec<Box<ParserError>>)
}

impl Expr {
    pub fn eval(&self, buf: &mut HashMap<String, i64>) -> Result<i64, ParserError> {
        match self {
            ConstExpr(value) => Ok(*value),
            Expr::NegExpr(expr) => {
                match expr.eval(buf) {
                    Ok(value) => Ok(-value),
                    Err(e) => Err(e)
                }
            },
            ParenthesisExpr(expr) => expr.eval(buf),
            BinaryExpr(l, op, r) => {
                match (l.eval(buf), r.eval(buf)) {
                    (Ok(l), Ok(r)) => {
                        Ok(match op {
                            Op::Plus => l + r,
                            Op::Minus => l - r,
                            Op::Times => l * r,
                            Op::Div => l / r,
                        })
                    }
                    (Err(r), Ok(_)) => Err(r),
                    (Ok(_), Err(err)) => Err(err),
                    (Err(err1), Err(err2)) => Err(MultipleError(vec![Box::new(err1), Box::new(err2)]))
                }
            }
            AssignmentExpr(name, value) => {
                let eval = value.eval(buf);
                match eval {
                    Ok(value) => {buf.insert(name.clone(), value);},
                    _ => {}
                }
                eval
            }
            IdentExpr(name) => {
                match buf.get(name) {
                    Some(value) => Ok(*value),
                    None => Err(UnknownVariable(name.clone()))
                }
            }
        }
    }
}

pub fn build_tree(tokens: &Vec<Token>) -> Option<Expr> {
    let mut parser = Parser::new(tokens);
    parser.parse_expression()
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::token::*;

    fn assert_ast_eval(text: &str, expected: i64) {
        let tokens = tokenize(&text.to_string());
        if let Some(ast) = build_tree(&tokens) {
            match ast.eval(&mut HashMap::new()) {
                Ok(value) => assert_eq!(value, expected),
                Err(_) => assert!(false)
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_ast_eval() {
        assert_ast_eval("1", 1);
        assert_ast_eval("1 + 1 + 1", 3);
        assert_ast_eval("1 * 1 * 1", 1);
        assert_ast_eval("1 + 2 * 3", 7);
        assert_ast_eval("2 * 3 + 1", 7);
        assert_ast_eval("2 * (3 + 1)", 8);
        assert_ast_eval("(2 * 3) + 1", 7);
        assert_ast_eval("1 + 1 + 1 + 1 + 1 + 1", 6);


        assert_ast_eval("-1", -1);
        assert_ast_eval("-1 + 1", 0);
        assert_ast_eval("-1 + 2 * 2", 3);
        assert_ast_eval("2 * 2 - 1", 3);
    }


}
