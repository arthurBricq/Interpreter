use std::collections::HashMap;

use crate::ast::declaration::Declaration;
use crate::ast::statement::StatementEval;
use crate::error::EvalError;

#[derive(Debug)]
pub struct Module {
    declarations: Vec<Declaration>,
}

impl Module {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self { declarations }
    }

    pub fn number_of_functions(&self) -> usize {
        self.declarations.iter().filter(|d| matches!(d, Declaration::Function(_, _, _))).count()
    }

    /// Returns a function by its name
    pub fn get_function(&self, name: &String) -> Option<&Declaration> {
        self.declarations.iter().find(|d| match d {
            Declaration::Function(fname, _, _) => fname == name
        })
    }

    /// Evaluate the `main` function
    pub fn run(&self) -> Result<StatementEval, EvalError> {
        match self.get_function(&"main".to_string()) {
            None => Err(EvalError::Error("Function main not found")),
            Some(main) => main.eval(&mut HashMap::new(), Some(&self))
        }
    }

    pub fn debug(&self) {
        for d in &self.declarations {
            println!("------");
            println!("{d:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ast::expression::Value::{BoolValue, IntValue};
    use crate::ast::statement::StatementEval;
    use crate::parser::Parser;
    use crate::token::tokenize;

    #[test]
    fn test_eval_main() {
        let text = crate::parser::tests::get_simple_file();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        let result = module.run();
        println!("{result:?}");
    }

    #[test]
    fn test_if_fonction_in_module() {
        let text = std::fs::read_to_string("TestData/if_else_loops.txt").unwrap();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();

        let bar = module.get_function(&"bar".to_string()).unwrap();
        let result = bar.eval(&mut HashMap::new(), Some(&module));
        assert_eq!(result, Ok(StatementEval::Return(IntValue(0))));

        let dog = module.get_function(&"dog".to_string()).unwrap();
        let result = dog.eval(&mut HashMap::new(), Some(&module));
        assert_eq!(result, Ok(StatementEval::Return(IntValue(0))));

        let cat = module.get_function(&"cat".to_string()).unwrap();
        let result = cat.eval(&mut HashMap::new(), Some(&module));
        assert_eq!(result, Ok(StatementEval::Return(IntValue(20))));
    }

    #[test]
    fn test_returns_true_or_false() {
        let text = std::fs::read_to_string("TestData/if_else_loops.txt").unwrap();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();

        let returns_true = module.get_function(&"returns_true".to_string()).unwrap();
        let result = returns_true.eval(&mut HashMap::new(), Some(&module));
        assert_eq!(result, Ok(StatementEval::Return(BoolValue(true))));

        let returns_false = module.get_function(&"returns_false".to_string()).unwrap();
        let result = returns_false.eval(&mut HashMap::new(), Some(&module));
        assert_eq!(result, Ok(StatementEval::Return(BoolValue(false))));
    }
    
    #[test]
    fn test_fibonnaci_function() {
        let text = std::fs::read_to_string("TestData/fibonacci.txt").unwrap();
        let tokens = tokenize(&text).unwrap();
        let mut parser = Parser::new(&tokens);
        
        let mut inputs = HashMap::new();
        
        let module = parser.parse_module();
        let func = module.get_function(&"fib".to_string()).unwrap();

        inputs.insert("n".to_string(), IntValue(0));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(0))));
        
        inputs.insert("n".to_string(), IntValue(1));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(1))));
        
        inputs.insert("n".to_string(), IntValue(2));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(1))));

        inputs.insert("n".to_string(), IntValue(3));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(2))));

        inputs.insert("n".to_string(), IntValue(4));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(3))));

        inputs.insert("n".to_string(), IntValue(5));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(5))));

        inputs.insert("n".to_string(), IntValue(6));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(8))));

        inputs.insert("n".to_string(), IntValue(10));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(55))));

        inputs.insert("n".to_string(), IntValue(15));
        assert_eq!(func.eval(&mut inputs, Some(&module)), Ok(StatementEval::Return(IntValue(610))));
    }
}