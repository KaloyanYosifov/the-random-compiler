use crate::operator::Operator;
use regex::Regex;
use std::fmt::Display;

const KEYWORDS: &'static [&str] = &[
    "if", "elif", "else", "while", "for", "return", "continue", "break", "int", "bool", "string",
    "char", "float",
];

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(Operator),
    Literal(String),
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
        for keyword in KEYWORDS {
            if keyword == &word {
                return true;
            }
        }

        return false;
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
            Self::Literal(value) => format!("STRING: {}", value),
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
            word if Self::is_string(word) => Self::Literal(word[1..word.len() - 1].into()),
            word if Self::is_number(word) => Self::Number(word.into()),
            word if word.len() == 1 => {
                match word.chars().next().unwrap().into() {
                    Self::Error(_) => Self::Identifier(word.to_owned()),
                    token => token 
                }
            },
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_can_create_keyword_tokens_from_word() {
        for keyword in KEYWORDS {
            let token: Token = (*keyword).into();
            assert_eq!(token, Token::Keyword((*keyword).to_owned()));
        }
    }

    #[rstest]
    #[case("+", Token::Operator(Operator::Plus))]
    #[case("-", Token::Operator(Operator::Minus))]
    #[case("*", Token::Operator(Operator::Mul))]
    #[case("/", Token::Operator(Operator::Div))]
    #[case("==", Token::Operator(Operator::Equal))]
    #[case("<", Token::Operator(Operator::Lesser))]
    #[case("<=", Token::Operator(Operator::LesserEqual))]
    #[case(">", Token::Operator(Operator::Greater))]
    #[case(">=", Token::Operator(Operator::GreaterEqual))]
    #[case("19", Token::Number("19".to_owned()))]
    #[case("19.5", Token::Number("19.5".to_owned()))]
    #[case("testing", Token::Identifier("testing".to_owned()))]
    #[case("test", Token::Identifier("test".to_owned()))]
    #[case("testing.testing_again", Token::Identifier("testing.testing_again".to_owned()))]
    #[case("\"Hello there\"", Token::Literal("Hello there".to_owned()))]
    #[case("=", Token::Assignment)]
    #[case("(", Token::Lparen)]
    #[case(")", Token::Rparen)]
    #[case("{", Token::LCurly)]
    #[case("}", Token::RCurly)]
    fn it_can_create_tokens_from_word(#[case] word: &str, #[case] expected: Token) {
        let token: Token = word.into();

        assert_eq!(token, expected);
    }

    #[rstest]
    #[case('+', Token::Operator(Operator::Plus))]
    #[case('-', Token::Operator(Operator::Minus))]
    #[case('*', Token::Operator(Operator::Mul))]
    #[case('/', Token::Operator(Operator::Div))]
    #[case('=', Token::Assignment)]
    #[case('(', Token::Lparen)]
    #[case(')', Token::Rparen)]
    #[case('{', Token::LCurly)]
    #[case('}', Token::RCurly)]
    fn it_can_create_tokens_from_character(#[case] character: char, #[case] expected: Token) {
        let token: Token = character.into();

        assert_eq!(token, expected);
    }
}
