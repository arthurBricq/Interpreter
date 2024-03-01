use crate::ast::statement::Statement;

/// A function argument currently only contains a string
#[derive(Debug)]
pub struct FnArg(pub String);

/// A declaration is the top-level element of a file: list of declaration
#[derive(Debug)]
pub enum Declaration {
    /// A function = name + list of expression (arguments) + list of statement
    Function(String, Vec<FnArg>, Statement)
}
