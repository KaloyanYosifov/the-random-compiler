use std::fmt::Display;

enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

enum Token {
    IDENTIFIER(String),
    KEYWORD(String),
    OPERATOR(Operator),
    LPAREN,
    RPAREN,
    SEMI,
    ASSIGNMENT,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_display = match self {
            Self::IDENTIFIER(id) => format!("IDENTIFIER: {}", id),
            Self::KEYWORD(key) => format!("KEYWORD: {}", key),
            Self::OPERATOR(operator) => format!("OPERATOR: {}", operator),
            Self::LPAREN => "LPAREN".to_owned(),
            Self::RPAREN => "RPAREN".to_owned(),
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

fn main() {
    let identifier_token = Token::IDENTIFIER("mistake".to_owned());
    let keyword_token = Token::KEYWORD("if".to_owned());
    println!(
        "Hello, Cool Compiler! Some examples: {} and {}",
        identifier_token, keyword_token
    );
}
