use crate::ast::expression::Expr;

#[derive(Debug)]
pub enum Statement {
    /// A statement of the type `expr;'
    SimpleStatement(Expr),
    /// A block of {statement}
    CompoundStatement(Vec<Box<Statement>>),
    /// A return statement, for functions
    Return(Expr),
}
