use std::io::Write;

use crate::shell::Shell;

mod ast;
mod parser;
mod shell;
mod token;

fn main() {
    let mut shell = Shell::new();
    shell.run()
}
