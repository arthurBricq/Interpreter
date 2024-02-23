use crate::ast::Expr::{
    AssignmentExpr, BinaryExpr, ConstExpr, IdentExpr, NegExpr, ParenthesisExpr,
};
use crate::ast::{Expr, Statement};
use crate::token::{Op, Token};

/// A struct to contain data related to parsing
///
/// Top Down Parser
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

/// Public API
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    /// An expression is something that is evaluated to something.
    /// (unlike statements that evaluates to nothing)
    pub fn parse_expression(&mut self) -> Option<Expr> {
        if let Some(assign) = self.parse_assignment_expr() {
            return Some(assign);
        } else if let Some(tmp) = self.parse_additive_expr() {
            return Some(tmp);
        }
        None
    }

    pub fn parse_statements(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        self.parse_one_statement(&mut statements);
        statements
    }
}

impl<'a> Parser<'a> {
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

    fn parse_one_statement(&mut self, fill_in: &mut Vec<Statement>) {
        if let Some(expr) = self.parse_expression() {
            if let Some(Token::SemiColon) = self.peek() {
                self.index += 1;
                fill_in.push(Statement::SimpleStatement(expr));
                // Potentially, there are many statements
                self.parse_one_statement(fill_in);
            }
        }
    }

    /// Matches "Ident = Something"
    fn parse_assignment_expr(&mut self) -> Option<Expr> {
        let checkpoint = self.index;
        if let Some(Token::Ident(name)) = self.consume() {
            if let Some(Token::Equal) = self.consume() {
                if let Some(expr) = self.parse_expression() {
                    return Some(AssignmentExpr(name.clone(), Box::new(expr)));
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
            if let Some(Token::TokenOp(y @ Op::Plus) | Token::TokenOp(y @ Op::Minus)) = self.peek()
            {
                self.index += 1;
                if let Some(right) = self.parse_additive_expr() {
                    return Some(BinaryExpr(Box::new(left), y, Box::new(right)));
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
            if let Some(Token::TokenOp(y @ Op::Times) | Token::TokenOp(y @ Op::Div)) = self.peek() {
                self.index += 1;
                if let Some(right) = self.parse_multiplicative_expr() {
                    return Some(BinaryExpr(Box::new(left), y, Box::new(right)));
                }
            } else {
                return Some(left);
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Matches constant, identifier or (expr) or -(primary)
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
                    return Some(ParenthesisExpr(Box::new(expr)));
                }
            }
        }
        self.set_index(checkpoint);
        // - Something
        if let Some(Token::TokenOp(Op::Minus)) = self.peek() {
            self.index += 1;
            if let Some(expr) = self.parse_primary_expr() {
                return Some(NegExpr(Box::new(expr)));
            }
        }
        None
    }
}

pub fn parse_expression(tokens: &Vec<Token>) -> Option<Expr> {
    let mut parser = Parser::new(tokens);
    parser.parse_expression()
}

pub fn parse_statements(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut parser = Parser::new(tokens);
    parser.parse_statements()
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr::{BinaryExpr, ConstExpr};
    use crate::ast::*;
    use crate::parser::{parse_expression, parse_statements};
    use crate::token::*;

    fn assert_ast(text: &str, expected: Expr) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        if let Some(ast) = parse_expression(&tokens) {
            assert_eq!(ast, expected);
        } else {
            assert!(false);
        }
    }

    fn assert_ast_with_text(text: &str, expected: &str) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        if let Some(ast) = parse_expression(&tokens) {
            assert_eq!(format!("{ast:?}"), expected);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_ast() {
        assert_ast(
            "1 + 2",
            BinaryExpr(Box::new(ConstExpr(1)), Op::Plus, Box::new(ConstExpr(2))),
        );
        assert_ast(
            "123 / 2",
            BinaryExpr(Box::new(ConstExpr(123)), Op::Div, Box::new(ConstExpr(2))),
        );
        assert_ast(
            "1 * 2",
            BinaryExpr(Box::new(ConstExpr(1)), Op::Times, Box::new(ConstExpr(2))),
        );
        assert_ast(
            "1 - 2",
            BinaryExpr(Box::new(ConstExpr(1)), Op::Minus, Box::new(ConstExpr(2))),
        );

        assert_ast_with_text(
            "(1+1)",
            "ParenthesisExpr(BinaryExpr(ConstExpr(1), Plus, ConstExpr(1)))",
        );
        assert_ast_with_text("(123+1) * 2 + 1", "BinaryExpr(BinaryExpr(ParenthesisExpr(BinaryExpr(ConstExpr(123), Plus, ConstExpr(1))), Times, ConstExpr(2)), Plus, ConstExpr(1))");

        assert_ast_with_text("a = 1", "AssignmentExpr(\"a\", ConstExpr(1))");
        assert_ast_with_text("a1 = 1", "AssignmentExpr(\"a1\", ConstExpr(1))");
        assert_ast_with_text(
            "a1 = (1+1)",
            "AssignmentExpr(\"a1\", ParenthesisExpr(BinaryExpr(ConstExpr(1), Plus, ConstExpr(1))))",
        );
        assert_ast_with_text("a", "IdentExpr(\"a\")");

        // To fix
        assert_ast_with_text(
            "1+1+1",
            "BinaryExpr(ConstExpr(1), Plus, BinaryExpr(ConstExpr(1), Plus, ConstExpr(1)))",
        );
        assert_ast_with_text(
            "1*1*1",
            "BinaryExpr(ConstExpr(1), Times, BinaryExpr(ConstExpr(1), Times, ConstExpr(1)))",
        );
    }

    #[test]
    fn test_parse_single_statement() {
        let text = "a=1;".to_string();
        let tokens = tokenize(&text);
        let statements = parse_statements(&tokens);
        assert_eq!(1, statements.len());
        println!("{statements:?}");
    }

    #[test]
    fn test_parse_multiple_statements() {
        let text = "a=1;b=1;c=a+b;".to_string();
        let tokens = tokenize(&text);
        let statements = parse_statements(&tokens);
        assert_eq!(3, statements.len());
        println!("{statements:#?}");
    }
}
