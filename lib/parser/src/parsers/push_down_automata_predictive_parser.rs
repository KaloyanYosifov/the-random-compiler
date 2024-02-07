use std::collections::HashMap;

use lexer::lexer::Lexer;

struct PushDownAutomataPredictiveParser {
    lexer: Lexer,
}

impl PushDownAutomataPredictiveParser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }
}
