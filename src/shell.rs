use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use crate::ast::{build_tree, Expr};
use crate::token::tokenize;

pub struct Shell {
    vars: HashMap<String, i64>
}

impl Shell {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new()
        }
    }

    fn eval(&mut self, ast: &Expr) -> i64 {
        ast.eval(&mut self.vars)
    }

    pub fn run(&mut self) {

        loop {
            // Shell parsing
            print!("  > ");
            let mut s = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }

            let tokens = tokenize(&s);
            if let Some(ast) = build_tree(&tokens) {
                let eval = self.eval(&ast);
                println!("   input = {s}");
                println!("   {tokens:?}");
                println!("  {ast:?}");
                println!("  {eval:?}");
            } else {
                println!("   Parsing Error")
            }
        }

    }
}