use lexer::{
    lexer::{Lexer, LexerError},
    token::{Token, TokenClass},
};
use thiserror::Error as ThisError;

type ParserResult = Result<ParseNode, ParserError>;

#[derive(Debug, Clone)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct ParseNode {
    pub loc: Loc,
    pub kind: String,
    pub value: Option<String>,
    pub children: Vec<Self>,
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

    fn is_next(&mut self, token: TokenClass) -> bool {
        if let Some(token_info) = self.lexer.peek() {
            token_info.token == token
        } else {
            false
        }
    }

    fn parse_statement(&mut self) -> ParserResult {
        let mut expression = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: "Statement".to_owned(),
            value: None,
            children: vec![],
        };

        // check first if we have an identifier next
        // if so, then we have a function call
        if self.is_next(TokenClass::Identifier) {
            let identifier = self.eat(TokenClass::Identifier)?;
            expression.loc = identifier.loc.clone();

            expression.children.push(identifier);
            expression.children.push(self.eat(TokenClass::Lparen)?);
            // parse expression
            expression.children.push(self.eat(TokenClass::Rparen)?);
            expression.children.push(self.eat(TokenClass::Semi)?);
        } else {
            let keyword = self.eat(TokenClass::Keyword)?;
            expression.loc = keyword.loc.clone();

            expression.children.push(keyword);
            expression.children.push(self.eat(TokenClass::Identifier)?);
            expression.children.push(self.eat(TokenClass::Assignment)?);
            expression.children.push(self.eat(TokenClass::Number)?);
            expression.children.push(self.eat(TokenClass::Semi)?);
        }

        Ok(expression)
    }

    fn parse_program(&mut self) -> ParserResult {
        let mut root = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: "Program".to_owned(),
            value: None,
            children: vec![],
        };

        root.children.push(self.parse_statement()?);
        // while let Some(_) = self.lexer.peek() {
        //     root.children.push(self.parse_expression()?);
        // }

        Ok(root)
    }
}

impl Parser {
    // create entire parse tree for now
    // TODO: make it streamable, we parse one at a time, for performance reasons
    pub fn parse(&mut self) -> ParserResult {
        self.parse_program()
    }
}

#[cfg(test)]
mod tests {
    // TODO: should we mock lexer, or it's fine for the unit test

    use super::*;

    #[test]
    fn it_can_parse_a_statement() {
        let lexer = Lexer::new("int a = 3;".to_owned());
        let mut parser = Parser::new(lexer);
        let parse_tree = parser.parse().unwrap();

        assert_eq!("Program".to_owned(), parse_tree.kind);
        assert_eq!(1, parse_tree.loc.line);
        assert_eq!(1, parse_tree.loc.column);
        assert!(parse_tree.value.is_none());

        let statement = parse_tree.children.get(0).unwrap();
        assert_eq!("Statement".to_owned(), statement.kind);
        assert_eq!(1, statement.loc.line);
        assert_eq!(1, statement.loc.column);
        assert!(parse_tree.value.is_none());

        let child = statement.children.get(0).unwrap();
        assert_eq!("Keyword".to_owned(), child.kind);
        assert_eq!(1, child.loc.line);
        assert_eq!(1, child.loc.column);
        assert_eq!("int", child.value.as_ref().unwrap());

        let child = statement.children.get(1).unwrap();
        assert_eq!("Identifier".to_owned(), child.kind);
        assert_eq!(1, child.loc.line);
        assert_eq!(5, child.loc.column);
        assert_eq!("a", child.value.as_ref().unwrap());

        let child = statement.children.get(2).unwrap();
        assert_eq!("Assignment".to_owned(), child.kind);
        assert_eq!(1, child.loc.line);
        assert_eq!(7, child.loc.column);
        assert!(parse_tree.value.is_none());

        let child = statement.children.get(3).unwrap();
        assert_eq!("Number".to_owned(), child.kind);
        assert_eq!(1, child.loc.line);
        assert_eq!(9, child.loc.column);
        assert_eq!("3", child.value.as_ref().unwrap());

        let child = statement.children.get(4).unwrap();
        assert_eq!("Semi".to_owned(), child.kind);
        assert_eq!(1, child.loc.line);
        assert_eq!(10, child.loc.column);
        assert!(parse_tree.value.is_none());
    }
}
