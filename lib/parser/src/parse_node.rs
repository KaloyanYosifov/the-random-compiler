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
    TokenClass(TokenClass),
}

impl From<&TokenClass> for NodeKind {
    fn from(token_class: &TokenClass) -> Self {
        Self::TokenClass(token_class.clone())
    }
}

impl From<TokenClass> for NodeKind {
    fn from(token_class: TokenClass) -> Self {
        Self::TokenClass(token_class)
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
        let kind = match &self.kind {
            NodeKind::TokenClass(tk) => tk.to_string(),
            v => v.to_string(),
        };

        if let Some(value) = &self.value {
            println!("{}{}: {}", pad_str, kind, value);
        } else {
            println!("{}{}", pad_str, kind);
        }

        for child in &self.children {
            child.inner_print_tree(padding + 2);
        }
    }
}
