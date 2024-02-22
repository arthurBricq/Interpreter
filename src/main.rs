use std::io::Write;

use crate::shell::Shell;

mod token;
mod ast;
mod shell;

fn main() {
    let mut shell = Shell::new();
    shell.run()
}
