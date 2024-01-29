use lexer::{
    lexer::{Lexer, LexerError, TokenInfo},
    token::Token,
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
    fn eat(&mut self, token: Token) -> Result<TokenInfo, ParserError> {
        let next_token = self.lexer.next()?;

        if next_token.token.is_equal_discrimnant(&token) {
            Ok(next_token)
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn eat_specific(&mut self, token: Token) -> Result<TokenInfo, ParserError> {
        let next_token = self.lexer.next()?;

        if next_token.token == token {
            Ok(next_token)
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
        let keyword = self.eat(Token::Keyword("".to_owned()))?;
        expression.children.push(ParseNode {
            loc: Loc {
                line: keyword.line,
                column: keyword.start_column,
            },
            value: keyword.token.extract_value(),
            kind: "Keyword".to_owned(),
            children: vec![],
        });

        let identifier = self.eat(Token::Identifier("".to_owned()))?;
        expression.children.push(ParseNode {
            loc: Loc {
                line: identifier.line,
                column: identifier.start_column,
            },
            value: identifier.token.extract_value(),
            kind: "Identifier".to_owned(),
            children: vec![],
        });

        let assignment = self.eat(Token::Assignment)?;
        expression.children.push(ParseNode {
            loc: Loc {
                line: assignment.line,
                column: assignment.start_column,
            },
            value: None,
            kind: "Assignment".to_owned(),
            children: vec![],
        });

        let number = self.eat(Token::Number("".to_owned()))?;
        expression.children.push(ParseNode {
            loc: Loc {
                line: number.line,
                column: number.start_column,
            },
            value: number.token.extract_value(),
            kind: "Number".to_owned(),
            children: vec![],
        });

        self.eat(Token::Semi)?;

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
