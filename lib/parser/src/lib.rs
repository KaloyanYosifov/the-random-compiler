use lexer::lexer::{Lexer, LexerError};
use lexer::token::Token;
use thiserror::Error as ThisError;

#[derive(Debug)]
pub struct ParseNode {
    value: String,
    next: Option<Box<Self>>,
}

pub struct Parser {
    root: ParseNode,
    lexer: Lexer,
}

#[derive(ThisError, Debug)]
pub enum ParserError {
    #[error("Lexer has failed!")]
    LexerError(#[from] LexerError),
    #[error("Unexpected token!")]
    UnexpectedToken,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            root: ParseNode {
                next: None,
                value: "Program".to_owned(),
            },
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<ParseNode, ParserError> {
        let mut root = ParseNode {
            value: "Program".to_owned(),
            next: None,
        };
        let mut next = &mut root;
        let info = self.lexer.next()?;

        match info.token {
            Token::Identifier(x) => {
                let new_r = Box::new(ParseNode {
                    value: "Identifier".to_owned(),
                    next: None,
                });
                next.next = Some(new_r);
                next = next.next.as_mut().unwrap();

                match self.lexer.next()?.token {
                    Token::Operator(s) if matches!(s, lexer::operator::Operator::Plus) => {
                        let new_r = Box::new(ParseNode {
                            value: "Plus".to_owned(),
                            next: None,
                        });
                        next.next = Some(new_r);
                        next = next.next.as_mut().unwrap();
                    }
                    _ => return Err(ParserError::UnexpectedToken),
                }
            }
            _ => return Err(ParserError::UnexpectedToken),
        };

        Ok(root)
    }
}
