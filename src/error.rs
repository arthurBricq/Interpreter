
#[derive(Debug)]
pub enum EvalError {
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
    TokensNotParsed
}
