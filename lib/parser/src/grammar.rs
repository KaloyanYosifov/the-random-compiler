use lexer::token::{Token, TokenClass, KEYWORDS};
use std::collections::HashMap;
use strum::{Display, EnumString};

#[derive(PartialEq, Eq, Hash, EnumString, Display)]
pub enum NonTerminal {
    #[strum(serialize = "P")]
    Program,
    #[strum(serialize = "S")]
    Statement,
    #[strum(serialize = "S'")]
    StatementPrime,
    #[strum(serialize = "A")]
    AssignmentStatement,
    #[strum(serialize = "E")]
    Expression,
    #[strum(serialize = "K")]
    Keyword,
    #[strum(serialize = "V")]
    Variable,
    #[strum(serialize = "Q")]
    Conditional,
    #[strum(serialize = "F")]
    ForLoop,
}

pub enum ProductionRuleSymbol {
    Token(Token),
    NonTerminal(NonTerminal),
    TokenClass(TokenClass),
    Empty,
}

pub type ProductionRule = Vec<ProductionRuleSymbol>;
pub type ProductionRules = Vec<ProductionRule>;
pub type GrammarTable = HashMap<NonTerminal, ProductionRules>;

pub struct Grammar {
    grammar: HashMap<NonTerminal, ProductionRules>,
}

impl Grammar {
    pub fn new() -> Self {
        let mut grammar = HashMap::new();

        Self::init_program_production_rules(&mut grammar);
        Self::init_statement_production_rules(&mut grammar);
        Self::init_assignment_statement_production_rules(&mut grammar);
        Self::init_expression_production_rules(&mut grammar);
        Self::init_conditional_production_rules(&mut grammar);
        Self::init_for_loop_production_rules(&mut grammar);
        Self::init_keyword_production_rules(&mut grammar);
        Self::init_variable_production_rules(&mut grammar);

        Self { grammar }
    }

    pub fn init_program_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Program,
            vec![vec![ProductionRuleSymbol::NonTerminal(
                NonTerminal::Statement,
            )]],
        );
    }

    pub fn init_statement_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Statement,
            vec![
                vec![
                    ProductionRuleSymbol::NonTerminal(NonTerminal::AssignmentStatement),
                    ProductionRuleSymbol::NonTerminal(NonTerminal::StatementPrime),
                ],
                vec![
                    ProductionRuleSymbol::NonTerminal(NonTerminal::Variable),
                    ProductionRuleSymbol::TokenClass(TokenClass::Lparen),
                    ProductionRuleSymbol::NonTerminal(NonTerminal::Expression),
                    ProductionRuleSymbol::TokenClass(TokenClass::Rparen),
                    ProductionRuleSymbol::NonTerminal(NonTerminal::StatementPrime),
                ],
            ],
        );
    }

    pub fn init_assignment_statement_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::AssignmentStatement,
            vec![vec![
                ProductionRuleSymbol::NonTerminal(NonTerminal::Keyword),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Variable),
                ProductionRuleSymbol::TokenClass(TokenClass::Assignment),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Expression),
                ProductionRuleSymbol::TokenClass(TokenClass::Semi),
            ]],
        );
    }

    pub fn init_conditional_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Conditional,
            vec![vec![
                ProductionRuleSymbol::NonTerminal(NonTerminal::Keyword),
                ProductionRuleSymbol::TokenClass(TokenClass::Lparen),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Expression),
                ProductionRuleSymbol::TokenClass(TokenClass::Rparen),
                ProductionRuleSymbol::TokenClass(TokenClass::LCurly),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Statement),
                ProductionRuleSymbol::TokenClass(TokenClass::RCurly),
            ]],
        );
    }

    pub fn init_for_loop_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Conditional,
            vec![vec![
                ProductionRuleSymbol::Token(Token::Keyword("for".to_owned())),
                ProductionRuleSymbol::TokenClass(TokenClass::Lparen),
                ProductionRuleSymbol::NonTerminal(NonTerminal::AssignmentStatement),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Expression),
                ProductionRuleSymbol::TokenClass(TokenClass::Semi),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Expression),
                ProductionRuleSymbol::TokenClass(TokenClass::Rparen),
                ProductionRuleSymbol::TokenClass(TokenClass::LCurly),
                ProductionRuleSymbol::NonTerminal(NonTerminal::Statement),
                ProductionRuleSymbol::TokenClass(TokenClass::RCurly),
            ]],
        );
    }

    pub fn init_expression_production_rules(_table: &mut GrammarTable) {
        todo!("Implement expression production rules!");
    }

    pub fn init_keyword_production_rules(table: &mut GrammarTable) {
        let production_rules = KEYWORDS
            .iter()
            .map(|keyword| {
                vec![ProductionRuleSymbol::Token(Token::Keyword(
                    keyword.to_string(),
                ))]
            })
            .collect();

        table.insert(NonTerminal::Keyword, production_rules);
    }

    pub fn init_variable_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Variable,
            vec![vec![ProductionRuleSymbol::TokenClass(
                TokenClass::Identifier,
            )]],
        );
    }
}
