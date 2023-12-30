use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

pub enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

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
            Self::SEMI => "SEMI".to_owned(),
            Self::ASSIGNMENT => "ASSIGNMENT".to_owned(),
        };

        write!(f, "{}", to_display)
    }
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

pub struct TokenInfo {
    line: usize,         // Would lines exceed 4 billion? :D
    start_column: usize, // Would horizontal characters exceed 4 billion? :D
    token: Token,
}

pub struct Lexer {
    line: usize,
    column: usize,
    buffer: BufReader<File>,
    current_line_iterator: Option<std::vec::IntoIter<char>>,
}

impl Lexer {
    pub fn new(path: &str) -> Result<Self, String> {
        match File::open(Path::new(&path)) {
            Ok(file) => Ok(Self {
                line: 0,
                column: 0,
                buffer: BufReader::new(file),
                current_line_iterator: None,
            }),
            _ => Err("File couldn't be opened!".to_owned()),
        }
    }
}

impl Lexer {
    fn read_next_line(&mut self) -> &mut Self {
        let mut line: String = String::from("");
        self.buffer.read_line(&mut line).unwrap();

        let chars = line.chars().collect::<Vec<_>>().into_iter();
        self.current_line_iterator = Some(chars);
        self.line += 1;
        self.column = 0;

        self
    }

    pub fn next(&mut self) -> TokenInfo {
        if let Some(iterator) = &mut self.current_line_iterator {
            if let Some(char) = iterator.next() {
                println!("{}", char);

                return TokenInfo {
                    line: self.line,
                    start_column: self.column,
                    token: Token::SEMI,
                };
            }

            self.read_next_line();

            return self.next();
        }

        self.read_next_line();

        self.next()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

