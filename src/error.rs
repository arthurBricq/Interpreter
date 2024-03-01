
#[derive(Debug, Eq, PartialEq)]
pub enum EvalError {
    NotImplemented,
    UnknownVariable(String),
    MultipleError(Vec<Box<EvalError>>),
}

#[derive(Debug)]
pub enum TokenError {
    UnknownChar(char)
}

#[derive(Debug)]
pub enum ParserError {
    /// The parser did not find any match
    UnknownSyntax,
    /// When a token is remaining after parsing is finished.
    TokensNotParsed, 
    ExpectedDifferentToken(&'static str),
    WrongFunctionArgumentList,
    WrongFunctionBody,
}
