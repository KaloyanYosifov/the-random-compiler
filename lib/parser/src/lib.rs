use lexer::{
    lexer::{Lexer, LexerError, TokenInfo},
    token::{Token, TokenClass},
};
use thiserror::Error as ThisError;

type ParserResult = Result<ParseNode, ParserError>;

#[derive(Debug)]
pub struct Loc {
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct ParseNode {
    loc: Loc,
    kind: String,
    value: Option<String>,
    children: Vec<Self>,
}

pub struct Parser {
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
        Self { lexer }
    }
}

impl Parser {
    fn eat(&mut self, token: TokenClass) -> Result<ParseNode, ParserError> {
        let token_info = self.lexer.next()?;

        if token_info.token == token {
            Ok(ParseNode {
                loc: Loc {
                    line: token_info.line,
                    column: token_info.start_column,
                },
                value: token_info.token.extract_value(),
                kind: token.to_string(),
                children: vec![],
            })
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn eat_specific(&mut self, token: Token) -> Result<ParseNode, ParserError> {
        let token_info = self.lexer.next()?;

        if token_info.token == token {
            Ok(ParseNode {
                loc: Loc {
                    line: token_info.line,
                    column: token_info.start_column,
                },
                value: token_info.token.extract_value(),
                kind: token_info.token.to_token_class().to_string(),
                children: vec![],
            })
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn parse_expression(&mut self) -> ParserResult {
        let mut expression = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: "Expression".to_owned(),
            value: None,
            children: vec![],
        };
        expression.children.push(self.eat(TokenClass::Keyword)?);
        expression.children.push(self.eat(TokenClass::Identifier)?);
        expression.children.push(self.eat(TokenClass::Assignment)?);
        expression.children.push(self.eat(TokenClass::Number)?);
        expression.children.push(self.eat(TokenClass::Semi)?);

        Ok(expression)
    }

    fn parse_program(&mut self) -> ParserResult {
        let mut root = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: "Program".to_owned(),
            value: None,
            children: vec![],
        };

        root.children.push(self.parse_expression()?);
        // while let Some(_) = self.lexer.peek() {
        //     root.children.push(self.parse_expression()?);
        // }

        Ok(root)
    }
}

impl Parser {
    pub fn parse(&mut self) -> ParserResult {
        self.parse_program()
    }
}
