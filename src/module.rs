use crate::ast::declaration::Declaration;

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
    pub fn get_function(&self, name: String) -> Option<&Declaration> {
        self.declarations.iter().find(|d| matches!(d, Declaration::Function(name, _, _)))
    }

    pub fn debug(&self) {
        for d in &self.declarations {
            println!("------");
            println!("{d:?}");
        }
    }
}
