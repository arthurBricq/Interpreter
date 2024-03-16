use crate::shell::Shell;

mod ast;
mod parser;
mod shell;
mod token;
mod error;
mod module;
mod std;

fn main() {
    let mut shell = Shell::new();
    shell.run()
}
