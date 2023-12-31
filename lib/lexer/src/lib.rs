use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    path::Path,
};

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Equal,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_display = match self {
            Self::Plus => "+".to_owned(),
            Self::Minus => "-".to_owned(),
            Self::Mul => "*".to_owned(),
            Self::Div => "/".to_owned(),
            Self::Equal => "==".to_owned(),
            Self::Lesser => "<".to_owned(),
            Self::LesserEqual => "<=".to_owned(),
            Self::Greater => ">".to_owned(),
            Self::GreaterEqual => ">=".to_owned(),
        };

        write!(f, "{}", to_display)
    }
}

impl From<&str> for Operator {
    fn from(word: &str) -> Self {
        match word {
            "==" => Self::Equal,
            "<=" => Self::LesserEqual,
            ">=" => Self::GreaterEqual,
            word => match word.chars().next().unwrap_or(' ') {
                '+' => Self::Plus,
                '-' => Self::Minus,
                '/' => Self::Div,
                '*' => Self::Mul,
                '>' => Self::Greater,
                '<' => Self::Lesser,
                _ => panic!("Please no!"),
            },
        }
    }
}

impl From<String> for Operator {
    fn from(word: String) -> Self {
        word.as_str().into()
    }
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(Operator),
    Lparen,
    Rparen,
    LCurly,
    RCurly,
    Semi,
    Assignment,
    Error(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_display = match self {
            Self::Identifier(id) => format!("IDENTIFIER: {}", id),
            Self::Keyword(key) => format!("KEYWORD: {}", key),
            Self::Operator(operator) => format!("OPERATOR: {}", operator),
            Self::Lparen => "(".to_owned(),
            Self::Rparen => ")".to_owned(),
            Self::LCurly => "{".to_owned(),
            Self::RCurly => "}".to_owned(),
            Self::Semi => ";".to_owned(),
            Self::Assignment => "=".to_owned(),
            Self::Error(error) => format!("Failed to convert to token: {}", error),
        };

        write!(f, "{}", to_display)
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            ';' => Self::Semi,
            '(' => Self::Lparen,
            ')' => Self::Rparen,
            '{' => Self::LCurly,
            '}' => Self::RCurly,
            '=' => Self::Assignment,
            _ => Self::Error(format!(
                "Failed to parse character to a token: {}",
                c.to_string()
            )),
        }
    }
}

impl From<&str> for Token {
    fn from(word: &str) -> Self {
        // TODO: Implement grammar (For now we do simple stuff)
        match word {
            word @ ("if" | "elif" | "else" | "while" | "for") => Self::Keyword(word.to_owned()),
            word @ ("+" | "-" | "/" | "*" | "==" | "<" | "<=" | ">" | ">=") => Self::Operator(word.into()),
            _ => Self::Identifier(word.to_owned()) // everything else is an identifier for now
            // _ => Self::Error(format!("Failed to convert word to token: {}", word)),
        }
    }
}

impl From<String> for Token {
    fn from(word: String) -> Self {
        word.as_str().into()
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

            while let Some(char) = peekable_iter.next() {
                self.column += 1;
                let next_char = peekable_iter.peek().unwrap_or(&' ');
                println!("Now: {}, next: {}", char, next_char);

                match char {
                    c if c.is_whitespace() => {
                        if word.len() > 0 {
                            break;
                        }

                        continue;
                    }
                    c if c == '=' && *next_char == '=' => {
                        return TokenInfo {
                            line: self.line,
                            start_column,
                            token: Token::Operator(Operator::Equal),
                        }
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
        let code = String::from("if (x == y) {");
        let mut lexer = Lexer::new(code);

        let token_info = lexer.next();
        assert_eq!(1, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Keyword(x) if x == "if"));

        let token_info = lexer.next();
        assert_eq!(4, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Lparen));

        let token_info = lexer.next();
        assert_eq!(5, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Identifier(x) if x == "x"));

        let token_info = lexer.next();
        assert_eq!(7, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Operator(x) if matches!(x, Operator::Equal)));

        let token_info = lexer.next();
        assert_eq!(9, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Identifier(x) if x == "y"));

        let token_info = lexer.next();
        assert_eq!(10, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Rparen));

        let token_info = lexer.next();
        assert_eq!(12, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::LCurly));
    }

    #[test]
    fn it_can_parse_multiline() {
        let code = String::from("if\nwhile\nfor");
        let mut lexer = Lexer::new(code);
        let token_info = lexer.next();

        assert_eq!(1, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Keyword(x) if x == "if"));

        let token_info = lexer.next();

        assert_eq!(1, token_info.start_column);
        assert_eq!(2, token_info.line);
        assert!(matches!(token_info.token, Token::Keyword(x) if x == "while"));

        let token_info = lexer.next();

        assert_eq!(1, token_info.start_column);
        assert_eq!(3, token_info.line);
        assert!(matches!(token_info.token, Token::Keyword(x) if x == "for"));
    }

    #[test]
    fn it_does_not_care_about_whitespaces() {
        let code = String::from("            if\n     \t    while\n");
        let mut lexer = Lexer::new(code);
        let token_info = lexer.next();

        assert_eq!(1, token_info.start_column);
        assert_eq!(1, token_info.line);
        assert!(matches!(token_info.token, Token::Keyword(x) if x == "if"));

        let token_info = lexer.next();

        assert_eq!(1, token_info.start_column);
        assert_eq!(2, token_info.line);
        assert!(matches!(token_info.token, Token::Keyword(x) if x == "while"));
    }
}
