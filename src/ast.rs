use crate::ast::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, ParenthesisExpr};
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


    // Matches "Ident = Something"
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

    /// Matches "Expr +/- Expr"
    fn parse_additive_expr(&mut self) -> Option<Expr> {
        let checkpoint = self.index;
        if let Some(left) = self.parse_multiplicative_expr() {
            if let Some(Token::TokenOp(y@ Op::Plus) | Token::TokenOp(y @ Op::Minus)) = self.peek() {
                self.index += 1;
                if let Some(right) = self.parse_multiplicative_expr() {
                    return Some(BinaryExpr(Box::new(left), y, Box::new(right)))
                }
            } else {
                return Some(left);
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Matches "Expr *// Expr"
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

    #[test]
    fn test_ast() {
        assert_ast("1 + 2", BinaryExpr(Box::new(ConstExpr(1)), Op::Plus, Box::new(ConstExpr(2))));
        assert_ast("123 / 2", BinaryExpr(Box::new(ConstExpr(123)), Op::Div, Box::new(ConstExpr(2))));
        assert_ast("1 * 2", BinaryExpr(Box::new(ConstExpr(1)), Op::Times, Box::new(ConstExpr(2))));
        assert_ast("1 - 2", BinaryExpr(Box::new(ConstExpr(1)), Op::Minus, Box::new(ConstExpr(2))));
        assert_ast("(1+2)", ParenthesisExpr(Box::new(BinaryExpr(Box::new(ConstExpr(1)), Op::Plus, Box::new(ConstExpr(2))))));
    }
}
