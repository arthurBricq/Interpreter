use std::io::{stdin, stdout, Write};
use crate::ast::build_tree;
use crate::shell::Shell;
use crate::token::tokenize;

mod token;
mod ast;
mod shell;

fn main() {
    let mut shell = Shell::new();
    shell.run()
}
