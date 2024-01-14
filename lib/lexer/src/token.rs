use crate::operator::Operator;
use regex::Regex;
use std::fmt::Display;

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
    pub fn is_special_char(char: char) -> bool {
        match char {
            ';' | '(' | ')' | '{' | '}' | '=' => true,
            _ => false,
        }
    }

    pub fn is_keyword(word: &str) -> bool {
        match word {
            "if" | "elif" | "else" | "while" | "for" | "return" | "continue" | "break" => true, // important
            "int" | "bool" | "string" | "char" | "float" => true, // primitives
            _ => false,
        }
    }

    pub fn is_string(word: &str) -> bool {
        let regex = Regex::new(r#"^(".*?")$"#).unwrap();

        regex.captures(word).is_some()
    }

    pub fn is_number(word: &str) -> bool {
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
            c if Operator::is_operator(&c.to_string()) => Self::Operator(c.to_string().into()),
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
