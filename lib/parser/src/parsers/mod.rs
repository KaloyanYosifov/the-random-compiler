use crate::parse_node::ParseNode;
use lexer::lexer::LexerError;
use thiserror::Error as ThisError;

mod recursive_descent_parser;

pub use recursive_descent_parser::RecursiveDescentParser;

pub type ParserResult = Result<ParseNode, ParserError>;

#[derive(ThisError, Debug)]
pub enum ParserError {
    #[error("Lexer has failed!")]
    LexerError(#[from] LexerError),
    #[error("Unexpected token: {0} actual was: {1}!")]
    UnexpectedToken(String, String),
}
