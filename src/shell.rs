use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use colored::Colorize;

use crate::ast::expression::*;
use crate::error::EvalError;
use crate::parser::parse_expression;
use crate::token::tokenize;

pub struct Shell {
    vars: HashMap<String, Value>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
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
                _ => self.interpret(&s)
            }
        }
    }

    fn eval(&mut self, ast: &Expr) -> Result<Value, EvalError> {
        ast.eval(&mut self.vars, None)
    }

    fn interpret(&mut self, text: &String) {
        match tokenize(text) {
            Ok(tokens) => {
                match parse_expression(&tokens) {
                    Ok(ast) => {
                        match self.eval(&ast) {
                            Ok(value) => println!("{value}"),
                            Err(e) => println!("{} {e:?}", "Error while evaluating: ".red()),
                        }
                    }
                    Err(e) => println!("{} {e:?}", "Error while parsing: ".red()),
                }
            }
            Err(err) => println!("{} {err:?}", "Error while tokenizing: ".red())
        }
    }
}
