use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use crate::ast::Expr;
use crate::error::ParserError;
use crate::parser::parse_expression;
use crate::token::tokenize;

pub struct Shell {
    vars: HashMap<String, i64>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    fn eval(&mut self, ast: &Expr) -> Result<i64, ParserError> {
        ast.eval(&mut self.vars)
    }

    pub fn run(&mut self) {
        loop {
            // Shell parsing
            print!(">>> ");
            let mut s = String::new();
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }

            match s.as_str() {
                "vars" => println!("{:?}", self.vars),
                _ => {
                    match tokenize(&s) {
                        Ok(tokens) => {
                            if let Some(ast) = parse_expression(&tokens) {
                                println!("{ast:?}");
                                match self.eval(&ast) {
                                    Ok(value) => println!("{value:?}"),
                                    Err(e) => println!("Error while evaluating: {e:?}"),
                                }
                            } else {
                                println!("Parsing Error")
                            }
                        }
                        Err(err) => println!("Error while tokenizing: {err:?}")
                    }
                }
            }
        }
    }
}
