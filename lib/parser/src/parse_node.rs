#[derive(Debug, Clone)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct ParseNode {
    pub loc: Loc,
    pub kind: String,
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
