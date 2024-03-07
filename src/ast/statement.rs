use std::collections::HashMap;

use crate::ast::expression::{Expr, Value};
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
    If(Expr, Box<Statement>, Option<Box<Statement>>)
}

impl Statement {
    pub fn eval(&self, inputs: &mut HashMap<String, Value>, module: Option<&Module>) -> Result<Value, EvalError> {
        match self {
            Statement::SimpleStatement(expr) => {
                match expr.eval(inputs, module) {
                    Ok(_) => return Ok(Value::None),
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
                        Ok(Value::None) => {}
                        Ok(result) => {
                            // If any of the statement returned anything, we return
                            // TODO there is probably a problem here.
                            return Ok(result)
                        }
                        Err(err) => return Err(err)
                    }
                }
                Ok(Value::None)
            }
            Statement::If(condition, body, else_statement)  => {
                if let Ok(cond) =  condition.eval(inputs, module) {
                    let test = match cond {
                        Value::IntValue(i) => i != 0,
                        Value::BoolValue(b) => b,
                        Value::None => false
                    };
                    
                    if test {
                        return body.eval(inputs, module)
                    } else if let Some(else_body) = else_statement {
                        return else_body.eval(inputs, module)
                    }
                };
                Ok(Value::None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::ast::expression::Value;

    use crate::error::EvalError;
    use crate::parser::{parse_statements, Parser};
    use crate::token::tokenize;

    fn assert_statement_eval(text: &str, expected: Result<Value, EvalError>) {
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let statements = parser.parse_statements();
        assert_eq!(1, statements.len());
        let block = &statements[0];
        let result = block.eval(&mut HashMap::new(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_statement_eval() {
        assert_statement_eval("a=1;", Ok(Value::None));
        assert_statement_eval("{a=1;a=2;}", Ok(Value::None));
        assert_statement_eval("{a=1; b=1; return a + b}", Ok(Value::IntValue(2)));
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
    
    #[test]
    fn test_if_evaluation() {
        let text = "if (1) {return 3;}";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        assert_eq!(statement.eval(&mut HashMap::new(), None), Ok(Value::IntValue(3)))
    }
    
    #[test]
    fn test_else_evaluation() {
        let text = "if (0) {return 3;} else {return 4}";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        assert_eq!(statement.eval(&mut HashMap::new(), None), Ok(Value::IntValue(4)))
    }

}
