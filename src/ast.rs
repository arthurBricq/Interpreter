use crate::ast::Ast::AssignmentExpr;
use crate::token::{Op, Token};

pub enum Ast {
    ConstExpr(i64),
    NegExpr(Box<Ast>),
    BinaryExpr(Box<Ast>, Op, Box<Ast>),
    AssignmentExpr(String, Box<Ast>),
    Ident(String),
    Statement(Box<Ast>)
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

    fn consume(&mut self) -> Option<Token> {
        let tkn = self.tokens.get(self.index).map(|x| x.clone());
        self.index += 1;
        tkn
    }

    /// An expression is something that is evaluated to something.
    /// (unlike statements that evaluates to nothing)
    fn parse_expression(&mut self) -> Option<Ast> {
        if let Some(assign) = self.parse_assignment_expr() {
            return Some(assign);
        } else if let Some(tmp) = self.parse_additive_expr() {
            return Some(tmp);
        }
        None
    }


    // Tries to match "Ident = Something"
    fn parse_assignment_expr(&mut self) -> Option<Ast> {
        let checkpoint = self.index;
        if let Some(Token::Ident(name)) = self.consume() {
            if let Some(Token::Equal) = self.consume().clone() {
                if let Some(expr) = self.parse_expression() {
                    return Some(AssignmentExpr(name.clone(), Box::new(expr)))
                }
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Parse additive expression and everything that has higher priority
    // fn parse_additive_expr(&mut self) -> Option<Ast> {
    //     if let Some(ast) = self.parse_multiplicative_expr() {
    //         // Is there an assignment ?
    //         if self.peek() == E
    //     }
    //
    //
    //
    //     /// TODO muptlie +
    //     /// TODO take care of -
    //     None
    // }

    fn parse_multiplicative_expr(&mut self) -> Option<Ast> {
        None
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

pub fn build_tree(tokens: &Vec<Token>) -> Option<Ast> {
    let mut parser = Parser::new(tokens);


    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ast() {

    }
}
