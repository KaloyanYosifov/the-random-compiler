use std::fmt::Display;

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
    pub fn is_operator(op: &str) -> bool {
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
