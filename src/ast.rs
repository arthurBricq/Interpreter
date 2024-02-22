use std::collections::HashMap;
use crate::ast::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, IdentExpr, ParenthesisExpr};
use crate::token::{Op, Token};

/// An expression is something that evaluates to something
#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    ConstExpr(i64),
    NegExpr(Box<Expr>),
    ParenthesisExpr(Box<Expr>),
    BinaryExpr(Box<Expr>, Op, Box<Expr>),
    AssignmentExpr(String, Box<Expr>),
    IdentExpr(String)
}

impl Expr {
    pub fn eval(&self, buf: &mut HashMap<String, i64>) -> i64 {
        match self {
            ConstExpr(value) => *value,
            Expr::NegExpr(expr) => -self.eval(buf),
            ParenthesisExpr(expr) => expr.eval(buf),
            BinaryExpr(l, op, r) => {
                match op {
                    Op::Plus => l.eval(buf) + r.eval(buf),
                    Op::Minus => l.eval(buf) - r.eval(buf),
                    Op::Times => l.eval(buf) * r.eval(buf),
                    Op::Div => l.eval(buf) / r.eval(buf),
                }
            },
            AssignmentExpr(name, value) => {
                let eval = value.eval(buf);
                buf.insert(name.clone(), eval);
                eval
            },
            IdentExpr(name) => buf.get(name).unwrap().clone()
        }
    }
}

/// A struct to contain data related to parsing
///
/// Top Down Parser
struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {tokens, index: 0}
    }

    /// Inspect current token
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.index).map(|x| x.clone())
    }

    /// Inspects current token and go forward
    fn consume(&mut self) -> Option<Token> {
        let tkn = self.tokens.get(self.index).map(|x| x.clone());
        self.index += 1;
        tkn
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn index(&self) -> usize {
        self.index
    }

    fn debug(&self) {
        println!("tokens = {:?}", self.tokens);
        println!("position = {:?}", self.index);
    }

    /// An expression is something that is evaluated to something.
    /// (unlike statements that evaluates to nothing)
    fn parse_expression(&mut self) -> Option<Expr> {
        if let Some(assign) = self.parse_assignment_expr() {
            return Some(assign);
        } else if let Some(tmp) = self.parse_additive_expr() {
            return Some(tmp);
        }
        None
    }

    /// Matches "Ident = Something"
    fn parse_assignment_expr(&mut self) -> Option<Expr> {
        let checkpoint = self.index;
        if let Some(Token::Ident(name)) = self.consume() {
            if let Some(Token::Equal) = self.consume() {
                if let Some(expr) = self.parse_expression() {
                    return Some(AssignmentExpr(name.clone(), Box::new(expr)))
                }
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Matches "Mul Expr +/- Mul Expr"
    fn parse_additive_expr(&mut self) -> Option<Expr> {
        let checkpoint = self.index;
        if let Some(left) = self.parse_multiplicative_expr() {
            if let Some(Token::TokenOp(y@ Op::Plus) | Token::TokenOp(y @ Op::Minus)) = self.peek() {
                self.index += 1;
                if let Some(right) = self.parse_additive_expr() {
                    return Some(BinaryExpr(Box::new(left), y, Box::new(right)))
                }
            } else {
                return Some(left);
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Matches "Primary * Expr" or "Primary"
    fn parse_multiplicative_expr(&mut self) -> Option<Expr> {
        let checkpoint = self.index;
        if let Some(left) = self.parse_primary_expr() {
            if let Some(Token::TokenOp(y@ Op::Times) | Token::TokenOp(y @ Op::Div)) = self.peek() {
                self.index += 1;
                if let Some(right) = self.parse_primary_expr() {
                    return Some(BinaryExpr(Box::new(left), y, Box::new(right)))
                }
            } else {
                return Some(left)
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Matches constant, identifier or (expr)
    fn parse_primary_expr(&mut self) -> Option<Expr> {
        // Constant
        if let Some(Token::Constant(value)) = self.peek() {
            self.index += 1;
            return Some(ConstExpr(value));
        }
        // Identifier
        if let Some(Token::Ident(s)) = self.peek() {
            self.index += 1;
            return Some(IdentExpr(s));
        }
        // Parenthesis
        let checkpoint = self.index;
        if let Some(Token::LPar) = self.consume() {
            if let Some(expr) = self.parse_expression() {
                if let Some(Token::RPar) = self.consume() {
                    return Some(ParenthesisExpr(Box::new(expr)))
                }
            }
        }
        self.set_index(checkpoint);
        None
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

    fn print_ast(text: &str) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        if let Some(ast) = build_tree(&tokens) {
            println!("{ast:?}");
        } else {
            println!("ast construction yield to None")
        }
    }

    fn assert_ast(text: &str, expected: Expr) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        if let Some(ast) = build_tree(&tokens) {
            println!("{ast:?}");
            assert_eq!(ast, expected);
        } else {
            println!("ast construction yield to None")
        }
    }

    fn assert_ast_with_text(text: &str, expected: &str) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        if let Some(ast) = build_tree(&tokens) {
            println!("{ast:?}");
            assert_eq!(format!("{ast:?}"), expected);
        } else {
            println!("ast construction yield to None");
            assert!(false);
        }
    }

    #[test]
    fn test_ast() {
        // assert_ast("1 + 2", BinaryExpr(Box::new(ConstExpr(1)), Op::Plus, Box::new(ConstExpr(2))));
        // assert_ast("123 / 2", BinaryExpr(Box::new(ConstExpr(123)), Op::Div, Box::new(ConstExpr(2))));
        // assert_ast("1 * 2", BinaryExpr(Box::new(ConstExpr(1)), Op::Times, Box::new(ConstExpr(2))));
        // assert_ast("1 - 2", BinaryExpr(Box::new(ConstExpr(1)), Op::Minus, Box::new(ConstExpr(2))));
        // assert_ast_with_text("(1+1)", "ParenthesisExpr(BinaryExpr(ConstExpr(1), Plus, ConstExpr(1)))");
        // assert_ast_with_text("(123+1) * 2 + 1", "BinaryExpr(BinaryExpr(ParenthesisExpr(BinaryExpr(ConstExpr(123), Plus, ConstExpr(1))), Times, ConstExpr(2)), Plus, ConstExpr(1))");
        //
        // assert_ast_with_text("a = 1", "AssignmentExpr(\"a\", ConstExpr(1))");
        // assert_ast_with_text("a1 = 1", "AssignmentExpr(\"a1\", ConstExpr(1))");
        // assert_ast_with_text("a1 = (1+1)", "AssignmentExpr(\"a1\", ParenthesisExpr(BinaryExpr(ConstExpr(1), Plus, ConstExpr(1))))");
        // assert_ast_with_text("a", "IdentExpr(\"a\")");

        // To fix
        assert_ast_with_text("1+1+1", "BinaryExpr(ConstExpr(1), Plus, BinaryExpr(ConstExpr(1), Plus, ConstExpr(1)))");
    }
}
