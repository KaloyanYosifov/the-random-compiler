use thiserror::Error as ThisError;

use crate::buffer::LexerBufferReader;
use crate::operator::*;
use crate::token::*;
use std::{
    fs::File,
    io::{BufReader, Cursor, Error as IOError},
    path::Path,
};

#[derive(ThisError, Debug)]
pub enum LexerError {
    #[error("Reached the end of the file!")]
    EndOfFileReached,
    #[error("Lexer was unable to read the next line of the file!")]
    FailedToReadNextLine(#[from] IOError),
    #[error("Could not open file: {0}")]
    CannotOpenFile(String),
}

#[derive(Debug)]
pub struct TokenInfo {
    pub line: usize,         // Would lines exceed 4 billion? :D
    pub start_column: usize, // Would horizontal characters exceed 4 billion? :D
    pub token: Token,
}

pub struct Lexer {
    line: usize,
    column: usize,
    cursor: LexerBufferReader,
    peeked: Option<TokenInfo>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            line: 1,
            column: 0,
            cursor: LexerBufferReader::new(Box::new(Cursor::new(code))),
            peeked: None,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, LexerError> {
        match File::open(Path::new(&path)) {
            Ok(file) => Ok(Self {
                line: 1,
                column: 0,
                cursor: LexerBufferReader::new(Box::new(BufReader::new(file))),
                peeked: None,
            }),
            _ => Err(LexerError::CannotOpenFile(path.to_owned())),
        }
    }
}

impl Lexer {
    pub fn next(&mut self) -> Result<TokenInfo, LexerError> {
        if let Some(token_info) = self.peeked.take() {
            return Ok(token_info);
        }

        if self.cursor.peek_char().is_none() {
            return Err(LexerError::EndOfFileReached);
        }

        let mut in_a_string = false; // temp fix to not break out of a string if it has spaces
        let mut word = String::from("");
        let mut start_column = self.column + 1;
        let start_line = self.line;

        while let Ok(char) = self.cursor.read_char() {
            if char == '\n' {
                self.line += 1;
                self.column = 0;

                break;
            }

            self.column += 1;
            let next_char = *self.cursor.peek_char().unwrap_or(&' ');
            let concatanated = format!("{}{}", char, next_char);

            match char {
                c if !in_a_string && c.is_whitespace() => {
                    if word.len() > 0 {
                        break;
                    }

                    start_column += 1;

                    continue;
                }
                // Check if concatanated with the next character we get an operator
                _ if !in_a_string && next_char != ' ' && Operator::is_operator(&concatanated) => {
                    self.column += 1;

                    self.cursor
                        .read_char()
                        .expect("We should have had a value here!");

                    return Ok(TokenInfo {
                        line: start_line,
                        start_column,
                        token: Token::Operator(concatanated.into()),
                    });
                }
                c if !in_a_string
                    && (Token::is_special_char(c) || Operator::is_operator(&c.to_string())) =>
                {
                    return Ok(TokenInfo {
                        line: start_line,
                        start_column,
                        token: c.into(),
                    });
                }
                c => {
                    word.push(c);

                    if c == '"' {
                        in_a_string = !in_a_string;
                    }

                    if !in_a_string && Token::is_special_char(next_char) {
                        break;
                    }
                }
            };
        }

        if word.len() == 0 {
            return self.next();
        }

        return Ok(TokenInfo {
            line: start_line,
            start_column,
            token: word.into(),
        });
    }

    // Implement peek, without going to the next position
    pub fn peek(&mut self) -> Option<&TokenInfo> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }

        match self.next() {
            Ok(token_info) => {
                self.peeked = Some(token_info);

                self.peeked.as_ref()
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_token_info {
        ($token:ident, $column:literal, $line:literal, $pattern:pat $(if $guard:expr)? $(,)?) => {
            let msg = format!("Token did not match. Actual: {:?}", $token.token);
            assert_eq!($column, $token.start_column);
            assert_eq!($line, $token.line);
            assert!(matches!(&$token.token, $pattern $(if $guard)?), "{}", msg);
        };

        ($token:expr, $column:literal, $line:literal, $pattern:pat $(if $guard:expr)? $(,)?) => {
            let token = $token.unwrap();
            let msg = format!("Token did not match. Actual: {:?}", token.token);
            assert_eq!($column, token.start_column);
            assert_eq!($line, token.line);
            assert!(matches!(&token.token, $pattern $(if $guard)?), "{}", msg);
        };
    }

    #[test]
    fn it_parses_if_statement() {
        let code = String::from("if (x == y) {");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 1, 1, Token::Keyword(x) if x == "if");
        assert_token_info!(lexer.next(), 4, 1, Token::Lparen);
        assert_token_info!(lexer.next(), 5, 1, Token::Identifier(x) if x == "x");
        assert_token_info!(lexer.next(), 7, 1, Token::Operator(x) if matches!(x, Operator::Equal));
        assert_token_info!(lexer.next(), 10, 1, Token::Identifier(x) if x == "y");
        assert_token_info!(lexer.next(), 11, 1, Token::Rparen);
        assert_token_info!(lexer.next(), 13, 1, Token::LCurly);
    }

    #[test]
    fn it_can_parse_multiline() {
        let code = String::from("if\nwhile\nfor");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 1, 1, Token::Keyword(x) if x == "if");
        assert_token_info!(lexer.next(), 1, 2, Token::Keyword(x) if x == "while");
        assert_token_info!(lexer.next(), 1, 3, Token::Keyword(x) if x == "for");
    }

    #[test]
    fn it_does_not_care_about_whitespaces() {
        let code = String::from("            if\n     \t    while\n");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 13, 1, Token::Keyword(x) if x == "if");
        assert_token_info!(lexer.next(), 11, 2, Token::Keyword(x) if x == "while");
    }

    #[test]
    fn it_can_parse_assignment_statement_with_string() {
        let code = String::from("string testing = \"Hello there\";");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 1, 1, Token::Keyword(x) if x == "string");
        assert_token_info!(lexer.next(), 8, 1, Token::Identifier(x) if x == "testing");
        assert_token_info!(lexer.next(), 16, 1, Token::Assignment);
        assert_token_info!(lexer.next(), 18, 1, Token::Literal(x) if x == "Hello there");
        assert_token_info!(lexer.next(), 31, 1, Token::Semi);
    }

    #[test]
    fn it_can_parse_an_assignment_statement_with_number() {
        let code = String::from("int testing = 33;");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 1, 1, Token::Keyword(x) if x == "int");
        assert_token_info!(lexer.next(), 5, 1, Token::Identifier(x) if x == "testing");
        assert_token_info!(lexer.next(), 13, 1, Token::Assignment);
        assert_token_info!(lexer.next(), 15, 1, Token::Number(x) if x == "33");
        assert_token_info!(lexer.next(), 17, 1, Token::Semi);
    }

    #[test]
    fn it_can_parse_expressions_in_assignment_statements() {
        let code = String::from("bool testing = 5 == 3.33;");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 1, 1, Token::Keyword(x) if x == "bool");
        assert_token_info!(lexer.next(), 6, 1, Token::Identifier(x) if x == "testing");
        assert_token_info!(lexer.next(), 14, 1, Token::Assignment);
        assert_token_info!(lexer.next(), 16, 1, Token::Number(x) if x == "5");
        assert_token_info!(lexer.next(), 18, 1, Token::Operator(x) if matches!(x, Operator::Equal));
        assert_token_info!(lexer.next(), 21, 1, Token::Number(x) if x == "3.33");
        assert_token_info!(lexer.next(), 25, 1, Token::Semi);
    }

    #[test]
    fn it_can_parse_a_function_call() {
        let code = String::from("sum(a + b);");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.next(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.next(), 4, 1, Token::Lparen);
        assert_token_info!(lexer.next(), 5, 1, Token::Identifier(x) if x == "a");
        assert_token_info!(lexer.next(), 7, 1, Token::Operator(x) if matches!(x, Operator::Plus));
        assert_token_info!(lexer.next(), 9, 1, Token::Identifier(x) if x == "b");
        assert_token_info!(lexer.next(), 10, 1, Token::Rparen);
        assert_token_info!(lexer.next(), 11, 1, Token::Semi);
    }

    #[test]
    fn it_throws_an_error_if_it_cannot_open_file() {
        let file = String::from("./some-file-that-does-not-exist.cl");

        if let Err(error) = Lexer::from_file(&file) {
            assert!(matches!(
                error,
                LexerError::CannotOpenFile(x) if x == file
            ));
        } else {
            assert!(false, "We should have failed!");
        }
    }

    #[test]
    fn it_throws_an_error_if_we_reached_end_of_file() {
        let code = String::from("");
        let mut lexer = Lexer::new(code);
        let error = lexer.next().unwrap_err();

        assert!(matches!(error, LexerError::EndOfFileReached));
    }

    #[test]
    fn it_can_peek_next_token() {
        let code = String::from("sum(a + b);");
        let mut lexer = Lexer::new(code);
        let peeked = lexer.peek().unwrap();

        assert_token_info!(peeked, 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.next(), 1, 1, Token::Identifier(x) if x == "sum");
    }

    #[test]
    fn it_can_peek_multiple_times() {
        let code = String::from("sum(a + b);");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.peek(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.peek(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.peek(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.peek(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.peek(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.next(), 1, 1, Token::Identifier(x) if x == "sum");
    }

    #[test]
    fn it_changes_next_peek_after_next_has_been_called() {
        let code = String::from("sum(a + b);");
        let mut lexer = Lexer::new(code);

        assert_token_info!(lexer.peek(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.next(), 1, 1, Token::Identifier(x) if x == "sum");
        assert_token_info!(lexer.peek(), 4, 1, Token::Lparen);
        assert_token_info!(lexer.peek(), 4, 1, Token::Lparen);
    }
}
