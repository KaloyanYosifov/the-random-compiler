use lexer::token::TokenClass;
use strum::Display;

#[derive(Debug, PartialEq, Eq, Display)]
pub enum NodeKind {
    Block,
    Program,
    Expression,

    // Statements
    Statement,
    ForLoopStatement,
    ReturnStatement,
    ControlFlowBlock,
    ConditionStatement,
    AssignmentStatement,

    // Functions
    Argument,
    Arguments,
    FunctionCall,
    FunctionDefinition,

    // Token classes
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

impl From<&TokenClass> for NodeKind {
    fn from(value: &TokenClass) -> Self {
        match value {
            TokenClass::Identifier => Self::Identifier,
            TokenClass::Keyword => Self::Keyword,
            TokenClass::Operator => Self::Operator,
            TokenClass::Literal => Self::Literal,
            TokenClass::Number => Self::Number,
            TokenClass::Boolean => Self::Boolean,
            TokenClass::Lparen => Self::Lparen,
            TokenClass::Rparen => Self::Rparen,
            TokenClass::LCurly => Self::LCurly,
            TokenClass::RCurly => Self::RCurly,
            TokenClass::Semi => Self::Semi,
            TokenClass::Comma => Self::Comma,
            TokenClass::Assignment => Self::Assignment,
            TokenClass::Error => Self::Error,
        }
    }
}

impl From<TokenClass> for NodeKind {
    fn from(value: TokenClass) -> Self {
        Self::from(&value)
    }
}

#[derive(Debug, Clone)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct ParseNode {
    pub loc: Loc,
    pub kind: NodeKind,
    pub value: Option<String>,
    pub children: Vec<Self>,
}

impl ParseNode {
    pub fn add_child(&mut self, node: ParseNode) {
        if self.children.len() == 0 {
            self.loc = node.loc.clone();
        }

        self.children.push(node);
    }

    pub fn print_tree(&self) {
        self.inner_print_tree(0)
    }

    fn inner_print_tree(&self, padding: i32) {
        let pad_str: String = (0..padding).map(|_| " ").collect();

        if let Some(value) = &self.value {
            println!("{}{}: {}", pad_str, self.kind, value);
        } else {
            println!("{}{}", pad_str, self.kind);
        }

        for child in &self.children {
            child.inner_print_tree(padding + 2);
        }
    }
}
