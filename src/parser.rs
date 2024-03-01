use crate::ast::declaration::{Declaration, FnArg};
use crate::ast::declaration::Declaration::Function;
use crate::ast::expression::Expr;
use crate::ast::expression::Expr::{AssignmentExpr, BinaryExpr, ConstExpr, IdentExpr, NegExpr, ParenthesisExpr};
use crate::ast::statement::Statement;
use crate::ast::statement::Statement::CompoundStatement;
use crate::error::ParserError;
use crate::error::ParserError::{ExpectedDifferentToken, UnknownSyntax, WrongFunctionArgumentList, WrongFunctionBody};
use crate::module::Module;
use crate::token::{Op, Token};

/// A struct to contain data related to parsing
///
/// Top-Down Parser
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
    pub fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        if let Some(assign) = self.parse_assignment_expr() {
            Ok(assign)
        } else if let Some(tmp) = self.parse_additive_expr() {
            Ok(tmp)
        } else {
            Err(UnknownSyntax)
        }
    }

    pub fn parse_statements(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        while let Some(stm) = self.parse_one_statement() {
            statements.push(stm);
        }
        statements
    }

    pub fn parse_module(&mut self) -> Module {
        let mut declarations = vec![];
        while let Ok(Some(ast)) = self.parse_declaration() {
            declarations.push(ast);
        }
        Module::new(declarations)
    }
}

impl<'a> Parser<'a> {
    /// Inspect current token
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.index).map(|x| x.clone())
    }

    fn is_finished(&self) -> bool {
        println!("{}, {}", self.index, self.tokens.len());
        self.index == self.tokens.len()
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


    /// Parse any kind of declaration
    fn parse_declaration(&mut self) -> Result<Option<Declaration>, ParserError> {
        self.parse_one_function()
    }

    /// Try to parse a function declaration
    fn parse_one_function(&mut self) -> Result<Option<Declaration>, ParserError> {
        if let Some(Token::Fn) = self.peek() {
            self.index += 1;
            if let Some(Token::Ident(name)) = self.peek() {
                self.index += 1;
                // Parse the list of arguments
                match self.parse_function_argument_list() {
                    Ok(arguments) => {
                        // Parse the body of the function
                        if let Some(body) = self.parse_compound_statement() {
                            return Ok(Some(Function(name, arguments, body)))
                        } else {
                            return Err(WrongFunctionBody)
                        }
                    }
                    Err(e) => return Err(e)
                }
            } else {
                return Err(ExpectedDifferentToken("Expecting an indent after function declaration"));
            }
        }
        Ok(None)
    }

    /// Try to parse the list of arguments in a function declaration
    fn parse_function_argument_list(&mut self) -> Result<Vec<FnArg>, ParserError> {
        if let Some(Token::LPar) = self.peek() {
            self.index += 1;
            let mut to_return = vec![];
            while let Some(token) = self.peek() {
                match token {
                    Token::Ident(name) => {
                        self.index += 1;
                        to_return.push(FnArg(name));
                    }
                    Token::RPar => {
                        self.index += 1;
                        return Ok(to_return);
                    }
                    Token::Comma => {
                        self.index += 1;
                    }
                    _ => {
                        return Err(WrongFunctionArgumentList)
                    }
                }
                

            }
            Ok(to_return)
        } else {
            Err(ExpectedDifferentToken("Expecting left par after function name"))
        }
    }

    fn parse_one_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Return) = self.peek() {
            self.index += 1;
            if let Ok(expr) = self.parse_expression() {
                return Some(Statement::Return(expr));
            }
            // TODO error handling
            return None;
        }
        if let Ok(expr) = self.parse_expression() {
            if let Some(Token::SemiColon) = self.peek() {
                self.index += 1;
                return Some(Statement::SimpleStatement(expr));
            }
        }
        if let Some(compound) = self.parse_compound_statement() {
            return Some(compound)
        }
        None
    }

    /// Parse all the statements included inside a { block }
    fn parse_compound_statement(&mut self) -> Option<Statement> {
        let checkpoint = self.index;
        if let Some(Token::LBracket) = self.peek() {
            self.index += 1;
            let mut statements = vec![];
            while let Some(stm) = self.parse_one_statement() {
                statements.push(Box::new(stm));
            }
            // Once there are no more statement being parsed, try to parse
            // a closing parenthesis.
            if let Some(Token::RBracket) = self.peek() {
                self.index += 1;
                return Some(CompoundStatement(statements));
            }
        }
        self.set_index(checkpoint);
        None
    }

    /// Matches "Ident = Something"
    fn parse_assignment_expr(&mut self) -> Option<Expr> {
        let checkpoint = self.index;
        if let Some(Token::Ident(name)) = self.consume() {
            if let Some(Token::Equal) = self.consume() {
                if let Ok(expr) = self.parse_expression() {
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
            if let Ok(expr) = self.parse_expression() {
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

/// Parse a single expression
pub fn parse_expression(tokens: &Vec<Token>) -> Result<Expr, ParserError> {
    let mut parser = Parser::new(tokens);
    match parser.parse_expression() {
        Ok(ast) => {
            if parser.is_finished() {
                Ok(ast)
            } else {
                Err(ParserError::TokensNotParsed)
            }
        }
        Err(err) => {
            Err(err)
        }
    }
}

/// Parse a list of statements
pub fn parse_statements(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut parser = Parser::new(tokens);
    parser.parse_statements()
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::ast::declaration::Declaration;
    use crate::ast::expression::Expr;
    use crate::ast::expression::Expr::{AssignmentExpr, BinaryExpr, ConstExpr};
    use crate::ast::statement::Statement;
    use crate::parser::{parse_expression, parse_statements, Parser};
    use crate::token::*;

    fn assert_ast(text: &str, expected: Expr) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        if let Ok(ast) = parse_expression(&tokens.unwrap()) {
            assert_eq!(ast, expected);
        } else {
            assert!(false);
        }
    }

    fn assert_ast_with_text(text: &str, expected: &str) {
        let tokens = tokenize(&text.to_string());
        print!("Building AST for <input> = <{text}>:   ");
        match parse_expression(&tokens.unwrap()) {
            Ok(ast) => { assert_eq!(format!("{ast:?}"), expected); }
            Err(err) => {
                println!("err={err:?}");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_simple_parenthesis() {
        assert_ast_with_text(
            "(1)",
            "ParenthesisExpr(ConstExpr(1))",
        );
    }

    #[test]
    fn test_basic_ast_expressions() {
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
        let statements = parse_statements(&tokens.unwrap());
        assert_eq!(1, statements.len());
        println!("{statements:?}");
    }

    #[test]
    fn test_parse_multiple_statements() {
        let text = "a=1;b=1;c=a+b;".to_string();
        let tokens = tokenize(&text);
        let statements = parse_statements(&tokens.unwrap());
        assert_eq!(3, statements.len());
        println!("{statements:#?}");
    }

    #[test]
    fn test_parse_coumpond_statements() {
        let text = "{a=1;b=1;c=a+b;a+b;}".to_string();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        if let Some(Statement::CompoundStatement(statements)) = parser.parse_compound_statement() {
            println!("result = {statements:?}");
            assert_eq!(statements.len(), 4);
            assert!(matches!(statements[0].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[1].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[2].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[3].as_ref(), Statement::SimpleStatement(BinaryExpr(_,Op::Plus, _))));
        } else {
            println!("failed");
            assert!(false);
        }
    }

    #[test]
    fn test_parse_coumpond_statements_new_line() {
        let text = "{a=1;\
        b=1;\
        c=a+b;\
        a+b;\
}".to_string();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        if let Some(Statement::CompoundStatement(statements)) = parser.parse_compound_statement() {
            println!("result = {statements:?}");
            assert_eq!(statements.len(), 4);
            assert!(matches!(statements[0].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[1].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[2].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[3].as_ref(), Statement::SimpleStatement(BinaryExpr(_,Op::Plus, _))));
        } else {
            println!("failed");
            assert!(false);
        }
    }

    #[test]
    fn test_parse_compound_with_return_statements() {
        let text = "{a=1; b=1; return a + b}".to_string();
        let tokens = tokenize(&text).unwrap();
        println!("{tokens:?}");
        let mut parser = Parser::new(&tokens);
        if let Some(Statement::CompoundStatement(statements)) = parser.parse_compound_statement() {
            println!("result = {statements:?}");
            assert_eq!(statements.len(), 3);
            assert!(matches!(statements[0].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[1].as_ref(), Statement::SimpleStatement(AssignmentExpr(_, _))));
            assert!(matches!(statements[2].as_ref(), Statement::Return(_)));
        } else {
            println!("failed");
            assert!(false);
        }
    }

    #[test]
    fn test_parse_function() {
        let text = "\
fn my_func_name(first, second) {
    a = first + second;
    return a + 1
}".to_string();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        match parser.parse_one_function() {
            Ok(Some(Declaration::Function(name, args, body))) => {
                println!("{name:?}");
                println!("{args:?}");
                println!("{body:?}");
                assert_eq!(name, "my_func_name".to_string());
                assert_eq!(args.len(), 2);
                assert_eq!(args[0].0, "first".to_string());
                assert_eq!(args[1].0, "second".to_string());
            }
            Ok(None) => assert!(false),
            Err(e) => {
                println!("Error = {e:?}");
                assert!(false);
            }
        }
        println!("{tokens:?}");
    }

    #[test]
    fn test_parse_function_no_arguments() {
        let text = "\
fn my_func_name() {
    return 1
}".to_string();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        match parser.parse_one_function() {
            Ok(Some(Declaration::Function(name, args, body))) => {
                println!("{name:?}");
                println!("{args:?}");
                println!("{body:?}");
                assert_eq!(name, "my_func_name".to_string());
                assert_eq!(args.len(), 0);
            }
            Ok(None) => assert!(false),
            Err(e) => {
                println!("Error = {e:?}");
                assert!(false);
            }
        }
        println!("{tokens:?}");
    }
    
    pub(crate) fn get_simple_file() -> String {
        std::fs::read_to_string("TestData/simple_file.txt").unwrap()
    }

    #[test]
    fn test_parse_file() {
        let text = get_simple_file();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let file = parser.parse_module();
        file.debug();
        assert_eq!(3, file.number_of_functions());
    }
}
