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
    pub fn eval(&self, inputs: &HashMap<String, i64>) -> Result<i64, EvalError> {
        match self {
            Declaration::Function(_name, args, body) => {
                // Is it normal that I don't use any of the args?
            }
        }
        
        Err(NotImplemented)
    }
}
#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::token::tokenize;

    #[test]
    fn test_eval_function() {
        let text = crate::parser::tests::get_simple_file();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let file = parser.parse_module();
        file.debug();
        assert_eq!(3, file.number_of_functions());
    }
}
