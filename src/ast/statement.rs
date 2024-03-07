use std::collections::HashMap;

use crate::ast::expression::Expr;
use crate::error::EvalError;
use crate::module::Module;

/// A statement is something that does not evaluate to something
#[derive(Debug)]
pub enum Statement {
    /// A statement of the type `expr;'
    SimpleStatement(Expr),
    /// A block of {statement}
    CompoundStatement(Vec<Statement>),
    /// A return statement, for functions
    Return(Expr),
    /// If statement
    /// The else is encapsulated as an optional statement
    If(Expr, Box<Statement>)
}

impl Statement {
    pub fn eval(&self, inputs: &mut HashMap<String, i64>, module: Option<&Module>) -> Result<Option<i64>, EvalError> {
        match self {
            Statement::SimpleStatement(expr) => {
                match expr.eval(inputs, module) {
                    Ok(_) => return Ok(None),
                    Err(err) => return Err(err)
                }
            }
            Statement::Return(expr) => {
                return match expr.eval(inputs, module) {
                    Ok(result) => Ok(result),
                    Err(err) => Err(err)
                }
            }
            Statement::CompoundStatement(statements) => {
                // All the new variables defined in the new scope are bound to remain in the scope
                // This forbid variable-side effect
                let mut copied_environment = inputs.clone();
                for stm in statements {
                    match stm.eval(&mut copied_environment, module) {
                        Ok(None) => {}
                        Ok(Some(result)) => {
                            // If we received a result, it means we have to leave
                            return Ok(Some(result))
                        }
                        Err(err) => return Err(err)
                    }
                }
                Ok(None)
            }
            Statement::If(condition, body)  => {
                if let Ok(Some(cond)) =  condition.eval(inputs, module) {
                    if (cond != 0) {
                        return body.eval(inputs, module)
                    }
                };
                Ok(None)
            }
        }
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
        let result = block.eval(&mut HashMap::new(), None);
        println!("{result:?}");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_statement_eval() {
        assert_statement_eval("a=1;", Ok(None));
        assert_statement_eval("{a=1;a=2;}", Ok(None));
        assert_statement_eval("{a=1; b=1; return a + b}", Ok(Some(2)));
    }
    #[test]
    fn test_error_when_using_variable_out_of_compound_scope() {
        // we want to test that a function does not have access to variables outside of its scope
        let file = "\
fn main() {
    a = 1;
    { b = 2; }
    return b;
}
        ".to_string();
        let tokens = tokenize(&file).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        println!("{module:?}");
        let result = module.run();
        println!("result = {result:?}");
        assert!(result.is_err());
    }

}
