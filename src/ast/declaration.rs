use std::collections::HashMap;
use crate::ast::statement::Statement;
use crate::error::EvalError;
use crate::error::EvalError::NotImplemented;

/// A function argument currently only contains a string
#[derive(Debug)]
pub struct FnArg(pub String);

/// A declaration is the top-level element of a file: list of declaration
#[derive(Debug)]
pub enum Declaration {
    /// A function = name + list of expression (arguments) + list of statement
    Function(String, Vec<FnArg>, Statement)
}

impl Declaration {
    /// Evaluate the output of the function based on the provided arguments
    pub fn eval(&self, inputs: &mut HashMap<String, i64>) -> Result<Option<i64>, EvalError> {
        match self {
            Declaration::Function(_name, args, body) => {
                // Is it normal that I don't use any of the args?
                return body.eval(inputs);
            }
        }
        
        Err(NotImplemented)
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::parser::Parser;
    use crate::token::tokenize;

    #[test]
    fn test_eval_function() {
        let text = crate::parser::tests::get_simple_file();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let file = parser.parse_module();
        assert_eq!(3, file.number_of_functions());
        
        let bar = file.get_function("bar".to_string()).unwrap();
        let result = bar.eval(&mut HashMap::new());
        assert_eq!(Ok(Some(3)), result);
        
        let foo = file.get_function("foo".to_string()).unwrap();
        let result = foo.eval(&mut HashMap::new());
        assert_eq!(Ok(Some(5)), result);
        
        // When running the add function without arguments, it's going to fail
        let add = file.get_function("add".to_string()).unwrap();
        let result = add.eval(&mut HashMap::new());
        assert!(matches!(result, Err(_)));

        // But we can run the add function with arguments, and it will return the sum of both
        let mut map = HashMap::new();
        map.insert("first".to_string(), 10);
        map.insert("second".to_string(), 2);
        let result = add.eval(&mut map);
        assert_eq!(Ok(Some(12)), result);
    }
}
