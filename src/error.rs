
#[derive(Debug)]
pub enum ParserError {
    UnknownVariable(String),
    MultipleError(Vec<Box<ParserError>>),
}

#[derive(Debug)]
pub enum TokenError {
    UnknownChar(char)
}