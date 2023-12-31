use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    path::Path,
};

#[derive(Debug)]
pub enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_display = match self {
            Self::PLUS => "+".to_owned(),
            Self::MINUS => "-".to_owned(),
            Self::MUL => "*".to_owned(),
            Self::DIV => "/".to_owned(),
        };

        write!(f, "{}", to_display)
    }
}

#[derive(Debug)]
pub enum Token {
    IDENTIFIER(String),
    KEYWORD(String),
    OPERATOR(Operator),
    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,
    SEMI,
    ASSIGNMENT,
    ERROR(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_display = match self {
            Self::IDENTIFIER(id) => format!("IDENTIFIER: {}", id),
            Self::KEYWORD(key) => format!("KEYWORD: {}", key),
            Self::OPERATOR(operator) => format!("OPERATOR: {}", operator),
            Self::LPAREN => "(".to_owned(),
            Self::RPAREN => ")".to_owned(),
            Self::LCURLY => "{".to_owned(),
            Self::RCURLY => "}".to_owned(),
            Self::SEMI => ";".to_owned(),
            Self::ASSIGNMENT => "=".to_owned(),
            Self::ERROR(error) => format!("Failed to convert to token: {}", error),
        };

        write!(f, "{}", to_display)
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            ';' => Token::SEMI,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LCURLY,
            '}' => Token::RCURLY,
            '=' => Token::ASSIGNMENT,
            _ => Token::ERROR(format!(
                "Failed to parse character to a token: {}",
                c.to_string()
            )),
        }
    }
}

impl From<String> for Token {
    fn from(word: String) -> Self {
        // TODO: Implement grammar (For now we do simple stuff)

        match word.as_str() {
            word @ ("if" | "elif" | "else" | "while" | "for") => Token::KEYWORD(word.to_owned()),
            _ => Token::ERROR(format!("Failed to convert word to token: {}", word)),
        }
    }
}

#[derive(Debug)]
pub struct TokenInfo {
    line: usize,         // Would lines exceed 4 billion? :D
    start_column: usize, // Would horizontal characters exceed 4 billion? :D
    token: Token,
}

#[derive(Debug)]
pub struct Lexer<T> {
    line: usize,
    column: usize,
    cursor: Cursor<T>,
    current_line_iterator: Option<std::vec::IntoIter<char>>,
}

impl Lexer<BufReader<File>> {
    pub fn from_file(path: &str) -> Result<Self, String> {
        match File::open(Path::new(&path)) {
            Ok(file) => Ok(Self {
                line: 0,
                column: 0,
                cursor: Cursor::new(BufReader::new(file)),
                current_line_iterator: None,
            }),
            _ => Err("File couldn't be opened!".to_owned()),
        }
    }
}

impl Lexer<String> {
    pub fn new(code: String) -> Self {
        Self {
            line: 0,
            column: 0,
            cursor: Cursor::new(code),
            current_line_iterator: None,
        }
    }
}

impl<T: AsRef<[u8]>> Lexer<T> {
    fn read_next_line(&mut self) -> &mut Self {
        let mut line: String = String::from("");
        self.cursor.read_line(&mut line).unwrap();

        let chars = line.chars().collect::<Vec<_>>().into_iter();
        self.current_line_iterator = Some(chars);
        self.line += 1;
        self.column = 0;

        self
    }

    pub fn next(&mut self) -> TokenInfo {
        if let Some(iterator) = &mut self.current_line_iterator {
            let mut peekable_iter = iterator.peekable();
            if peekable_iter.peek().is_none() {
                self.read_next_line();

                return self.next();
            }

            let mut word = String::from("");
            let start_column = self.column + 1;

            for char in peekable_iter {
                self.column += 1;

                match char {
                    ' ' | '\n' => {
                        if word.len() > 0 {
                            break;
                        }

                        continue;
                    }
                    c @ (';' | '(' | ')' | '{' | '}' | '=') => {
                        // TODO: add check to peek next character to see if we have another =
                        return TokenInfo {
                            line: self.line,
                            start_column,
                            token: c.into(),
                        };
                    }
                    c => word.push(c),
                };
            }

            return TokenInfo {
                line: self.line,
                start_column,
                token: word.into(),
            };
        }

        self.read_next_line();

        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_if_statement() {
        let code = String::from("if (x == y)");
        let mut lexer = Lexer::new(code);
        let token_info = lexer.next();

        assert_eq!(1, token_info.start_column);
        assert_eq!(1, token_info.line);
        match token_info.token {
            Token::KEYWORD(lexeme) => assert_eq!("if", lexeme),
            _ => assert!(false, "Invalid token"),
        }

        let next_token_info = lexer.next();

        assert_eq!(4, next_token_info.start_column);
        assert_eq!(1, next_token_info.line);
        assert!(
            matches!(next_token_info.token, Token::LPAREN),
            "Invalid token!"
        );
    }
}
