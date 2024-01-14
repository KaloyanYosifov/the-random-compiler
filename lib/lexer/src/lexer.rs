use thiserror::Error as ThisError;

use crate::operator::*;
use crate::token::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Error as IOError},
    iter::Peekable,
    path::Path,
    vec::IntoIter,
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
    line: usize,         // Would lines exceed 4 billion? :D
    start_column: usize, // Would horizontal characters exceed 4 billion? :D
    token: Token,
}

trait LineReader {
    fn read_next_line(&mut self, buf: &mut String) -> std::io::Result<usize>;
}

impl LineReader for BufReader<File> {
    fn read_next_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        self.read_line(buf)
    }
}

impl LineReader for Cursor<String> {
    fn read_next_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        self.read_line(buf)
    }
}

pub struct Lexer {
    line: usize,
    column: usize,
    cursor: Box<dyn LineReader>,
    current_line_iterator: Option<Peekable<IntoIter<char>>>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            line: 0,
            column: 0,
            cursor: Box::new(Cursor::new(code)),
            current_line_iterator: None,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, LexerError> {
        match File::open(Path::new(&path)) {
            Ok(file) => Ok(Self {
                line: 0,
                column: 0,
                cursor: Box::new(BufReader::new(file)),
                current_line_iterator: None,
            }),
            _ => Err(LexerError::CannotOpenFile(path.to_owned())),
        }
    }
}

impl Lexer {
    fn read_next_line(&mut self) -> Result<(), LexerError> {
        let mut line: String = String::from("");
        self.cursor.read_next_line(&mut line)?;

        if line.len() == 0 {
            return Err(LexerError::EndOfFileReached);
        }

        let chars = line.chars().collect::<Vec<_>>().into_iter().peekable();
        self.current_line_iterator = Some(chars);
        self.line += 1;
        self.column = 0;

        Ok(())
    }

    pub fn next(&mut self) -> Result<TokenInfo, LexerError> {
        if let Some(iterator) = &mut self.current_line_iterator {
            if iterator.peek().is_none() {
                self.read_next_line()?;

                return self.next();
            }

            let mut in_a_string = false; // temp fix to not break out of a string if it has spaces
            let mut word = String::from("");
            let mut start_column = self.column + 1;

            while let Some(char) = iterator.next() {
                self.column += 1;
                let next_char = *iterator.peek().unwrap_or(&' ');
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
                    _ if !in_a_string
                        && next_char != ' '
                        && Operator::is_operator(&concatanated) =>
                    {
                        self.column += 1;
                        iterator.next();

                        return Ok(TokenInfo {
                            line: self.line,
                            start_column,
                            token: Token::Operator(concatanated.into()),
                        });
                    }
                    c if !in_a_string
                        && (Token::is_special_char(c) || Operator::is_operator(&c.to_string())) =>
                    {
                        return Ok(TokenInfo {
                            line: self.line,
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
                line: self.line,
                start_column,
                token: word.into(),
            });
        }

        self.read_next_line()?;

        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_token_info {
        ($token:ident, $column:literal, $line:literal, $pattern:pat $(if $guard:expr)? $(,)?) => {
            let msg = format!("Token did not match. Actual: {:?}", token.token);
            assert_eq!($column, $token.start_column);
            assert_eq!($line, $token.line);
            assert!(matches!($token.token, $pattern $(if $guard)?), "{}", msg);
        };

        ($token:expr, $column:literal, $line:literal, $pattern:pat $(if $guard:expr)? $(,)?) => {
            let token = $token.unwrap();
            let msg = format!("Token did not match. Actual: {:?}", token.token);
            assert_eq!($column, token.start_column);
            assert_eq!($line, token.line);
            assert!(matches!(token.token, $pattern $(if $guard)?), "{}", msg);
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
        assert_token_info!(lexer.next(), 18, 1, Token::String(x) if x == "Hello there");
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
}
