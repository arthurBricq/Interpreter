use std::collections::HashMap;
use crate::ast::expression::Value;

use crate::ast::statement::Statement;
use crate::error::EvalError;
use crate::module::Module;

/// A function argument currently only contains a string
#[derive(Debug)]
pub struct FnArg(pub String);

/// A declaration is the top-level element of a file.
/// A file is list of declaration
#[derive(Debug)]
pub enum Declaration {
    /// A function = name + list of expression (arguments) + list of statement
    Function(String, Vec<FnArg>, Statement)
}

impl Declaration {
    /// Evaluate the output of the function based on the provided arguments
    /// Inputs are the inputs of the function
    pub fn eval(&self, inputs: &mut HashMap<String, Value>, module: Option<&Module>) -> Result<Value, EvalError> {
        return match self {
            Declaration::Function(_name, _args, body) => {
                // When evaluating a function, we must 
                // `body` is the compound statement of the function
                body.eval(inputs, module)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::ast::expression::Value;
    use crate::ast::expression::Value::IntValue;

    use crate::parser::Parser;
    use crate::token::tokenize;

    #[test]
    fn test_dummy_eval_function() {
        let text = crate::parser::tests::get_simple_file();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        
        let bar = module.get_function(&"bar".to_string()).unwrap();
        let result = bar.eval(&mut HashMap::new(), None);
        assert_eq!(Ok(IntValue(3)), result);
        
        let foo = module.get_function(&"foo".to_string()).unwrap();
        let result = foo.eval(&mut HashMap::new(), None);
        assert_eq!(Ok(IntValue(5)), result);
        
        // When running the add function without arguments, it's going to fail
        let add = module.get_function(&"add".to_string()).unwrap();
        let result = add.eval(&mut HashMap::new(), None);
        assert!(matches!(result, Err(_)));

        // But we can run the add function with arguments, and it will return the sum of both
        let mut map = HashMap::new();
        map.insert("first".to_string(), IntValue(10));
        map.insert("second".to_string(), IntValue(2));
        let result = add.eval(&mut map, None);
        assert_eq!(Ok(IntValue(12)), result);
        println!("{map:?}");
    }
    
    #[test]
    fn test_error_when_running_function_with_variable_defined_out_of_scope() {
        // we want to test that a function does not have access to variables outside of its scope
        let file = "\
fn foo() {
    return a;
}

fn main() {
    a = 1;
    return foo();
}
        ".to_string();
        let tokens = tokenize(&file).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        let result = module.run();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_success_when_passing_argument_to_functions() {
        // we want to test that a function does not have access to variables outside of its scope
        let file = "\
fn foo(a) {
    return a;
}

fn main() {
    a = 1;
    return foo(a);
}
        ".to_string();
        let tokens = tokenize(&file).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        let result = module.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), IntValue(1));
    }


    #[test]
    fn test_error_when_not_passing_argument() {
        let text = crate::parser::tests::get_simple_file();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        let pass = module.get_function(&"passthrough".to_string()).unwrap();
        assert!(matches!(pass.eval(&mut HashMap::new(), Some(&module)), Err(_)));
    }


    #[test]
    fn test_recursive_function() {
        let text = "\
fn recursive(n) {
    if (n == 0) {return 0;}
    return recursive(n - 1);
}
        ";
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        let func = module.get_function(&"recursive".to_string()).unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("n".to_string(), Value::IntValue(0));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(IntValue(0)));

        inputs.insert("n".to_string(), Value::IntValue(1));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(IntValue(0)));

        inputs.insert("n".to_string(), Value::IntValue(10));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(IntValue(0)));
    }

    #[test]
    fn test_sum_of_function_call() {
        let text = "\
fn foo(n) {
    return 2 * n;
}

fn main() {
    return foo(1) + foo(2);
}
        ";
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        println!("{module:?}");
        let result = module.run();
        println!("{result:?}");
        assert_eq!(result, Ok(IntValue(6)));
    }
}
