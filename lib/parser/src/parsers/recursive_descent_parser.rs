use crate::parse_node::{Loc, NodeKind, ParseNode};
use lexer::{
    lexer::Lexer,
    operator::Operator,
    token::{Token, TokenClass},
};

use super::{ParserError, ParserResult};

pub struct RecursiveDescentParser {
    lexer: Lexer,
}

impl RecursiveDescentParser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }
}

impl RecursiveDescentParser {
    fn eat(&mut self, token: &TokenClass) -> ParserResult {
        let peeked = self.lexer.peek();
        let mut node = None;
        let mut actual_token = String::from("Unknown");

        if let Some(token_info) = peeked {
            actual_token = token_info.token.to_string();

            if &token_info.token == token {
                node = Some(ParseNode {
                    loc: Loc {
                        line: token_info.line,
                        column: token_info.start_column,
                    },
                    value: token_info.token.extract_value(),
                    kind: token.into(),
                    children: vec![],
                });
            }
        }

        if let Some(node) = node {
            self.lexer.next()?;

            Ok(node)
        } else {
            Err(ParserError::UnexpectedToken(
                token.to_string(),
                actual_token,
            ))
        }
    }

    fn eat_any_of(&mut self, tokens: &[TokenClass]) -> ParserResult {
        for token in tokens {
            if let Ok(node) = self.eat(&token) {
                return Ok(node);
            }
        }

        let mut buffer = String::from("");

        for token in tokens {
            if buffer.len() != 0 {
                buffer.push_str(" or ");
            }

            buffer.push_str(&token.to_string());
        }

        let mut actual_token = String::from("Unknown");

        if let Some(token_info) = self.lexer.peek() {
            actual_token = token_info.token.to_string();
        }

        Err(ParserError::UnexpectedToken(buffer, actual_token))
    }

    fn eat_exact(&mut self, token: &Token) -> Result<ParseNode, ParserError> {
        let token_info = self.lexer.next()?;

        if &token_info.token == token {
            Ok(ParseNode {
                loc: Loc {
                    line: token_info.line,
                    column: token_info.start_column,
                },
                value: token_info.token.extract_value(),
                kind: token_info.token.to_token_class().into(),
                children: vec![],
            })
        } else {
            Err(ParserError::UnexpectedToken(
                token.to_string(),
                token_info.token.to_string(),
            ))
        }
    }

    fn is_next_exact(&mut self, token: &Token) -> bool {
        if let Some(token_info) = self.lexer.peek() {
            &token_info.token == token
        } else {
            false
        }
    }

    fn is_next(&mut self, token: &TokenClass) -> bool {
        if let Some(token_info) = self.lexer.peek() {
            &token_info.token == token
        } else {
            false
        }
    }

    fn is_next_exact_any_of(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.is_next_exact(token) {
                return true;
            }
        }

        return false;
    }

    fn is_next_any_of(&mut self, tokens: &[TokenClass]) -> bool {
        for token in tokens {
            if self.is_next(token) {
                return true;
            }
        }

        return false;
    }
}

impl RecursiveDescentParser {
    fn parse_expression(&mut self) -> ParserResult {
        let mut expression = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::Expression,
            value: None,
            children: vec![],
        };

        if self.is_next(&TokenClass::Lparen) {
            let l_paren = self.eat(&TokenClass::Lparen)?;
            expression.loc = l_paren.loc.clone();

            expression.add_child(l_paren);
            expression.add_child(self.parse_expression()?);
            expression.add_child(self.eat(&TokenClass::Rparen)?);
        } else {
            expression.add_child(self.eat_any_of(&[
                TokenClass::Identifier,
                TokenClass::Boolean,
                TokenClass::Number,
                TokenClass::Literal,
            ])?);
        }

        if self.is_next_exact(&Token::Operator(Operator::Increment)) {
            expression.add_child(self.eat(&TokenClass::Operator)?);
        } else if self.is_next(&TokenClass::Operator) {
            expression.add_child(self.eat(&TokenClass::Operator)?);
            expression.add_child(self.parse_expression()?);
        }

        Ok(expression)
    }

    fn parse_block(&mut self) -> ParserResult {
        let mut block = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::Block,
            value: None,
            children: vec![],
        };

        block.add_child(self.eat(&TokenClass::LCurly)?);

        while !self.is_next(&TokenClass::RCurly) {
            block.add_child(self.parse_statement()?);
        }

        block.add_child(self.eat(&TokenClass::RCurly)?);

        Ok(block)
    }

    fn parse_control_flow_block(&mut self) -> ParserResult {
        let mut block = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::ControlFlowBlock,
            value: None,
            children: vec![],
        };

        block.add_child(self.eat(&TokenClass::Lparen)?);
        block.add_child(self.parse_expression()?);
        block.add_child(self.eat(&TokenClass::Rparen)?);
        block.add_child(self.parse_block()?);

        Ok(block)
    }

    fn parse_for_loop_statement(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::ForLoopStatement,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat(&TokenClass::Keyword)?);
        statement.add_child(self.eat(&TokenClass::Lparen)?);
        statement.add_child(self.parse_assignment_statement()?);
        statement.add_child(self.parse_expression()?);
        statement.add_child(self.eat(&TokenClass::Semi)?);
        statement.add_child(self.parse_expression()?);
        statement.add_child(self.eat(&TokenClass::Rparen)?);
        statement.add_child(self.parse_block()?);

        Ok(statement)
    }

    fn parse_condition_statement(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::ConditionStatement,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat(&TokenClass::Keyword)?);
        statement.add_child(self.parse_control_flow_block()?);

        Ok(statement)
    }

    fn parse_assignment_statement(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::AssignmentStatement,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat(&TokenClass::Keyword)?);
        statement.add_child(self.eat(&TokenClass::Identifier)?);
        statement.add_child(self.eat(&TokenClass::Assignment)?);

        while !self.is_next(&TokenClass::Semi) {
            statement.add_child(self.parse_expression()?);
        }

        statement.add_child(self.eat(&TokenClass::Semi)?);

        Ok(statement)
    }

    fn parse_argument(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::Argument,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat(&TokenClass::Keyword)?);
        statement.add_child(self.eat(&TokenClass::Identifier)?);

        Ok(statement)
    }

    fn parse_arguments(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::Arguments,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat(&TokenClass::Lparen)?);

        while !self.is_next(&TokenClass::Rparen) {
            statement.add_child(self.parse_argument()?);
        }

        statement.add_child(self.eat(&TokenClass::Rparen)?);

        Ok(statement)
    }

    fn parse_function_definition(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::FunctionDefinition,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat_exact(&Token::Keyword("fn".to_owned()))?);
        statement.add_child(self.eat(&TokenClass::Identifier)?);
        statement.add_child(self.parse_arguments()?);
        statement.add_child(self.eat_exact(&Token::Operator(Operator::Pointer))?);
        statement.add_child(self.eat(&TokenClass::Keyword)?);
        statement.add_child(self.parse_block()?);

        Ok(statement)
    }

    fn parse_return_statement(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::ReturnStatement,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat_exact(&Token::Keyword("return".to_owned()))?);

        while !self.is_next(&TokenClass::Semi) {
            statement.add_child(self.parse_expression()?);
        }

        statement.add_child(self.eat(&TokenClass::Semi)?);

        Ok(statement)
    }

    fn parse_keyword_statement(&mut self) -> ParserResult {
        let conditional_statements = [
            Token::Keyword("if".to_owned()),
            Token::Keyword("while".to_owned()),
        ];

        match true {
            _ if self.is_next_exact_any_of(&conditional_statements) => {
                self.parse_condition_statement()
            }
            _ if self.is_next_exact(&Token::Keyword("for".to_owned())) => {
                self.parse_for_loop_statement()
            }
            _ if self.is_next_exact(&Token::Keyword("fn".to_owned())) => {
                self.parse_function_definition()
            }
            _ if self.is_next_exact(&Token::Keyword("return".to_owned())) => {
                self.parse_return_statement()
            }
            _ => self.parse_assignment_statement(),
        }
    }

    fn parse_function_call_statement(&mut self) -> ParserResult {
        let mut statement = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::FunctionCall,
            value: None,
            children: vec![],
        };

        statement.add_child(self.eat(&TokenClass::Identifier)?);
        statement.add_child(self.eat(&TokenClass::Lparen)?);
        statement.add_child(self.parse_expression()?);
        statement.add_child(self.eat(&TokenClass::Rparen)?);
        statement.add_child(self.eat(&TokenClass::Semi)?);

        Ok(statement)
    }

    fn parse_statement(&mut self) -> ParserResult {
        if self.is_next(&TokenClass::Keyword) {
            self.parse_keyword_statement()
        } else {
            self.parse_function_call_statement()
        }
    }

    fn parse_program(&mut self) -> ParserResult {
        let mut root = ParseNode {
            loc: Loc { line: 1, column: 1 },
            kind: NodeKind::Program,
            value: None,
            children: vec![],
        };

        while let Some(_) = self.lexer.peek() {
            root.add_child(self.parse_statement()?);
        }

        Ok(root)
    }
}

impl RecursiveDescentParser {
    // create entire parse tree for now
    // TODO: make it streamable, we parse one at a time, for performance reasons
    pub fn parse(&mut self) -> ParserResult {
        self.parse_program()
    }
}
