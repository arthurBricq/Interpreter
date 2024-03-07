use std::collections::HashMap;
use crate::ast::declaration::Declaration;
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
    pub fn run(&self) -> Result<Option<i64>, EvalError> {
        let main = self.get_function(&"main".to_string()).unwrap();
        main.eval(&mut HashMap::new(), Some(&self))
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
        assert_eq!(result, Ok(Some(0)));
    }
}