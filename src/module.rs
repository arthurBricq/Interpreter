use crate::ast::Declaration;

#[derive(Debug)]
pub struct Module {
    declarations: Vec<Declaration>
}

impl Module {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self { declarations }
    }

    pub fn number_of_functions(&self) -> usize {
        self.declarations.iter().filter(|d| matches!(d, Declaration::Function(_, _, _))).count()
    }
    
    pub fn debug(&self) {
        for d in &self.declarations {
            println!("------");
            println!("{d:?}");
        }
    }
}
