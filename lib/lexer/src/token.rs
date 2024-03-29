use crate::operator::Operator;
use regex::Regex;
use std::fmt::Display;
use strum::Display as StrumDisplay;

pub const KEYWORDS: &'static [&str] = &[
    "if", "elif", "else", "while", "for", "return", "continue", "break", "int", "bool", "string",
    "char", "float", "fn"
];

#[derive(PartialEq, Eq, Debug, StrumDisplay, Hash, Clone)]
pub enum TokenClass {
    Identifier,
    Keyword,
    Operator,
    Literal,
    Number,
    Boolean,
    Lparen,
    Rparen,
    LCurly,
    RCurly,
    Semi,
    Comma,
    Assignment,
    Error,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(Operator),
    Literal(String),
    Number(String),
    Boolean(String),
    Lparen,
    Rparen,
    LCurly,
    RCurly,
    Semi,
    Comma,
    Assignment,
    Error(String),
}

impl Token {
    pub fn is_special_char(char: char) -> bool {
        match char {
            ';' | '(' | ')' | '{' | '}' | '=' | ',' => true,
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

    pub fn is_boolean(word: &str) -> bool {
        let regex = Regex::new(r#"^true|false$"#).unwrap();

        regex.captures(word).is_some()
    }

    pub fn is_equal_discrimnant(&self, token: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(token)
    }

    pub fn to_token_class(&self) -> TokenClass {
        match &self {
            Self::Identifier(_) => TokenClass::Identifier,
            Self::Keyword(_) => TokenClass::Keyword,
            Self::Operator(_) => TokenClass::Operator,
            Self::Literal(_) => TokenClass::Literal,
            Self::Number(_) => TokenClass::Number,
            Self::Boolean(_) => TokenClass::Boolean,
            Self::Lparen => TokenClass::Lparen,
            Self::Rparen => TokenClass::Rparen,
            Self::LCurly => TokenClass::LCurly,
            Self::RCurly => TokenClass::RCurly,
            Self::Semi => TokenClass::Semi,
            Self::Comma => TokenClass::Comma,
            Self::Assignment => TokenClass::Assignment,
            Self::Error(_) => TokenClass::Error,
        }
    }

    pub fn extract_value(&self) -> Option<String> {
        match self {
            Self::Identifier(value) 
                | Self::Keyword(value) 
                | Self::Literal(value) 
                | Self::Error(value) 
                | Self::Number(value)
                | Self::Boolean(value)
                => Some(value.to_owned()),
            Self::Operator(value) => Some(value.to_string()),
            _ => None,
        }
    }
}

impl PartialEq<TokenClass> for Token {
    fn eq(&self, other: &TokenClass) -> bool {
        &self.to_token_class() == other
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
            Self::Boolean(value) => format!("Boolean: {}", value),
            Self::Lparen => "(".to_owned(),
            Self::Rparen => ")".to_owned(),
            Self::LCurly => "{".to_owned(),
            Self::RCurly => "}".to_owned(),
            Self::Semi => ";".to_owned(),
            Self::Comma => ",".to_owned(),
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
            ',' => Self::Comma,
            _ => Self::Error(format!(
                "Failed to parse character to a token: {}",
                c.to_string()
            )),
        }
    }
}

impl From<&str> for Token {
    fn from(word: &str) -> Self {
        match word {
            word if Self::is_keyword(word) => Self::Keyword(word.to_owned()),
            word if Operator::is_operator(word) => Self::Operator(word.into()),
            word if Self::is_boolean(word) => Self::Boolean(word.into()),
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
    #[case("int", Token::Keyword("int".to_owned()))]
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

    #[rstest]
    #[case("+", TokenClass::Operator)]
    #[case("19", TokenClass::Number)]
    #[case("testing", TokenClass::Identifier)]
    #[case("int", TokenClass::Keyword)]
    #[case("\"Hello there\"", TokenClass::Literal)]
    #[case("=", TokenClass::Assignment)]
    #[case("(", TokenClass::Lparen)]
    #[case(")", TokenClass::Rparen)]
    #[case("{", TokenClass::LCurly)]
    #[case("}", TokenClass::RCurly)]
    fn it_can_compare_a_token_and_a_token_class(#[case] word: &str, #[case] expected: TokenClass) {
        let token: Token = word.into();

        assert_eq!(token, expected);
        assert_eq!(token.to_token_class(), expected);
    }

    #[test]
    fn it_can_check_the_kind_of_the_tokens_without_the_value() {
        let token = Token::Keyword("test".to_string());
        let token2 = Token::Keyword("testing".to_owned());

        assert_ne!(token, token2);
        assert!(token.is_equal_discrimnant(&token2));
    }

    #[test]
    fn it_can_check_the_kind_of_the_tokens_without_the_value_and_return_false_if_the_tokens_are_of_different_type() {
        let token = Token::Keyword("test".to_owned());
        let token2 = Token::Identifier("testing".to_owned());

        assert_ne!(token, token2);
        assert!(!token.is_equal_discrimnant(&token2));
    }

    #[test]
    fn it_returns_false_if_token_and_token_class_do_not_match() {
        let token = Token::Keyword("test".to_owned());
        let token_class = TokenClass::Identifier;

        assert_ne!(token, token_class);
        assert_ne!(token.to_token_class(), token_class);
    }
}
