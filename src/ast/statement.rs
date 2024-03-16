use std::collections::HashMap;
use std::io::read_to_string;

use crate::ast::expression::{Expr, Value};
use crate::error::EvalError;
use crate::error::EvalError::Error;
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
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    /// Loops
    Loop(Box<Statement>),
    /// break is a statement since it does not execute to a value but to a side effect
    Break
}

#[derive(Debug, PartialEq, Eq)]
/// Holds the result of a statement's runtime evaluation
pub enum StatementEval {
    Return(Value),
    Break,
    None
}

impl Statement {
    
    fn eval_statement_list(inputs: &mut HashMap<String, Value>, module: Option<&Module>, statements: &Vec<Statement>) -> Result<StatementEval, EvalError> {
        for stm in statements {
            match stm.eval(inputs, module) {
                Ok(StatementEval::None) => {}
                Ok(StatementEval::Break) => return Ok(StatementEval::Break),
                Ok(StatementEval::Return(result)) => {
                    // If any of the statement returned anything, we return
                    // TODO there is probably a problem here.
                    return Ok(StatementEval::Return(result))
                }
                Err(err) => return Err(err)
            }
        }
        Ok(StatementEval::None)
    }
    
    
    pub fn eval(&self, inputs: &mut HashMap<String, Value>, module: Option<&Module>) -> Result<StatementEval, EvalError> {
        match self {
            Statement::SimpleStatement(expr) => {
                match expr.eval(inputs, module) {
                    Ok(_) => return Ok(StatementEval::None),
                    Err(err) => return Err(err)
                }
            }
            Statement::Return(expr) => {
                return match expr.eval(inputs, module) {
                    Ok(result) => Ok(StatementEval::Return(result)),
                    Err(err) => Err(err)
                }
            }
            Statement::CompoundStatement(statements) => {
                // All the new variables defined in the new scope are bound to remain in the scope
                // This forbid variable-side effect
                let mut copied_environment = inputs.clone();
                Self::eval_statement_list(&mut copied_environment, module, statements)
            }
            Statement::If(condition, body, else_statement)  => {
                match condition.eval(inputs, module) {
                    Ok(cond) => {
                        let test = match cond {
                            Value::IntValue(i) => i != 0,
                            Value::BoolValue(b) => b,
                            Value::None => return Err(EvalError::Error("'None' can't be casted to bool")),
                            Value::List(_) => return Err(EvalError::Error("List can't be casted to bool"))
                        };

                        if test {
                            body.eval(inputs, module)
                        } else if let Some(else_body) = else_statement {
                            else_body.eval(inputs, module)
                        } else { 
                            Ok(StatementEval::None)
                        }
                    }
                    Err(err) => Err(err)
                }
            }
            Statement::Loop(body) => {
                // We know that the body is necessary a compound statement
                // Unfortunately, it is not possible to call `
                match body.as_ref() {
                    Statement::CompoundStatement(statements) => {
                        while let Ok(result) = Self::eval_statement_list(inputs, module, statements) {
                            match result {
                                StatementEval::Break => {
                                    return Ok(StatementEval::None)
                                }
                                _ => {}
                            }
                        }
                        
                    }
                    _ => return Err(Error("A loop statement can only be associated with a compound statement."))
                }
                
                Ok(StatementEval::None)
            }
            Statement::Break => {
                Ok(StatementEval::Break)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::ast::expression::Value;
    use crate::ast::statement::StatementEval;

    use crate::error::EvalError;
    use crate::parser::{parse_statements, Parser};
    use crate::token::tokenize;

    fn assert_statement_eval(text: &str, expected: Result<StatementEval, EvalError>) {
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
        assert_statement_eval("a=1;", Ok(StatementEval::None));
        assert_statement_eval("{a=1;a=2;}", Ok(StatementEval::None));
        assert_statement_eval("{a=1; b=1; return a + b}", Ok(StatementEval::Return(Value::IntValue(2))));
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
        assert_eq!(statement.eval(&mut HashMap::new(), None), Ok(StatementEval::Return(Value::IntValue(3))))
    }
    
    #[test]
    fn test_else_evaluation() {
        let text = "if (0) {return 3;} else {return 4}";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        assert_eq!(statement.eval(&mut HashMap::new(), None), Ok(StatementEval::Return(Value::IntValue(4))))
    }
    
    #[test]
    fn test_if_evaluation_with_undefined_var() {
        let text = "if (n) {return 3;}";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        let result = statement.eval(&mut HashMap::new(), None);
        println!("{result:?}");
        assert!(matches!(result, Err(_)))
    }
    
    #[test]
    fn test_return_statement_with_addition() {
        let text = "{return 1 + 1}";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        println!("Getting ready");
        let result = statement.eval(&mut HashMap::new(), None);
        println!("{result:?}");
        assert_eq!(result, Ok(StatementEval::Return(Value::IntValue(2))));
    }
    
    #[test]
    fn test_return_inside_compound() {
        let text = "
{
    i = 0;
    i = i + 1;
    return i;
}
        ";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        let result = statement.eval(&mut HashMap::new(), None);
        println!("{result:?}");
        assert_eq!(result, Ok(StatementEval::Return(Value::IntValue(1))));
    }
    
    #[test]
    fn test_simple_loop_eval() {
        let text = "
{
    i = 0;
    loop {
        i = i + 1;
        if (i == 10) { break; }
    }
    return i;
}
        ";
        let tokens = tokenize(&text.to_string());
        let ast = parse_statements(&tokens.unwrap());
        let statement = &ast[0];
        let result = statement.eval(&mut HashMap::new(), None);
        println!("{result:?}");
        assert_eq!(result, Ok(StatementEval::Return(Value::IntValue(10))));
    }

}
