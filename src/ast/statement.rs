use std::collections::HashMap;

use crate::ast::expression::Expr;
use crate::error::EvalError;
use crate::error::EvalError::NotImplemented;

#[derive(Debug)]
pub enum Statement {
    /// A statement of the type `expr;'
    SimpleStatement(Expr),
    /// A block of {statement}
    CompoundStatement(Vec<Box<Statement>>),
    /// A return statement, for functions
    Return(Expr),
}

impl Statement {
    pub fn eval(&self, inputs: &mut HashMap<String, i64>) -> Result<Option<i64>, EvalError> {
        match self {
            Statement::SimpleStatement(expr) => {
                match expr.eval(inputs) {
                    Ok(_) => return Ok(None),
                    Err(err) => return Err(err)
                }
            }
            Statement::Return(expr) => {
                return match expr.eval(inputs) {
                    Ok(result) => Ok(Some(result)),
                    Err(err) => Err(err)
                }
            }
            Statement::CompoundStatement(statements) => {
                for stm in statements {
                    match stm.eval(inputs) {
                        Ok(None) => {}
                        Ok(Some(result)) => {
                            // If we received a result, it means we have to leave
                            return Ok(Some(result))
                        }
                        Err(err) => return Err(err)
                    }
                }
                return Ok(None)
            }
        }
        Err(NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::error::EvalError;
    use crate::parser::Parser;
    use crate::token::tokenize;

    fn assert_statement_eval(text: &str, expected: Result<Option<i64>, EvalError>) {
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let statements = parser.parse_statements();
        assert_eq!(1, statements.len());
        let block = &statements[0];
        let result = block.eval(&mut HashMap::new());
        println!("{result:?}");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_statement_eval() {
        assert_statement_eval("a=1;", Ok(None));
        assert_statement_eval("{a=1;a=2;}", Ok(None));
        assert_statement_eval("{a=1; b=1; return a + b}", Ok(Some(2)));
    }
}
