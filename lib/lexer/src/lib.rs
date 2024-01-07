use regex::Regex;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Cursor},
    iter::Peekable,
    path::Path,
    vec::IntoIter,
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

impl Operator {
    fn is_operator(op: &str) -> bool {
        match op {
            "+" | "-" | "/" | "*" | "==" | "<" | "<=" | ">" | ">=" => true,
            _ => false,
        }
    }
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
    String(String),
    Number(String),
    Lparen,
    Rparen,
    LCurly,
    RCurly,
    Semi,
    Assignment,
    Error(String),
}

impl Token {
    fn is_special_char(char: char) -> bool {
        match char {
            ';' | '(' | ')' | '{' | '}' | '=' => true,
            _ => false,
        }
    }

    fn is_keyword(word: &str) -> bool {
        match word {
            "if" | "elif" | "else" | "while" | "for" | "return" | "continue" | "break" => true, // important
            "int" | "bool" | "string" | "char" | "float" => true, // primitives
            _ => false,
        }
    }

    fn is_string(word: &str) -> bool {
        let regex = Regex::new(r#"^(".*?")$"#).unwrap();

        regex.captures(word).is_some()
    }

    fn is_number(word: &str) -> bool {
        let regex = Regex::new(r#"^(\d+(\.\d+)?)$"#).unwrap();

        regex.captures(word).is_some()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_display = match self {
            Self::Identifier(id) => format!("IDENTIFIER: {}", id),
            Self::Keyword(key) => format!("KEYWORD: {}", key),
            Self::Operator(operator) => format!("OPERATOR: {}", operator),
            Self::String(value) => format!("STRING: {}", value),
            Self::Number(value) => format!("NUMBER: {}", value),
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
            word if Self::is_keyword(word) => Self::Keyword(word.to_owned()),
            word if Operator::is_operator(word) => Self::Operator(word.into()),
            word if Self::is_string(word) => Self::String(word[1..word.len() - 1].into()),
            word if Self::is_number(word) => Self::Number(word.into()),
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
    current_line_iterator: Option<Peekable<IntoIter<char>>>,
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

        let chars = line.chars().collect::<Vec<_>>().into_iter().peekable();
        self.current_line_iterator = Some(chars);
        self.line += 1;
        self.column = 0;

        self
    }

    pub fn next(&mut self) -> TokenInfo {
        if let Some(iterator) = &mut self.current_line_iterator {
            if iterator.peek().is_none() {
                self.read_next_line();

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

                        return TokenInfo {
                            line: self.line,
                            start_column,
                            token: Token::Operator(concatanated.into()),
                        };
                    }
                    c if !in_a_string && Token::is_special_char(c)
                        || Operator::is_operator(&c.to_string()) =>
                    {
                        return TokenInfo {
                            line: self.line,
                            start_column,
                            token: c.into(),
                        };
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

    macro_rules! assert_token_info {
        ($token:ident, $column:literal, $line:literal, $pattern:pat $(if $guard:expr)? $(,)?) => {
            let msg = format!("Token did not match. Actual: {:?}", token.token);
            assert_eq!($column, $token.start_column);
            assert_eq!($line, $token.line);
            assert!(matches!($token.token, $pattern $(if $guard)?), "{}", msg);
        };

        ($token:expr, $column:literal, $line:literal, $pattern:pat $(if $guard:expr)? $(,)?) => {
            let token = $token;
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
}
