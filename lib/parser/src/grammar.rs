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
    #[strum(serialize = "K")]
    Keyword,
}

pub enum ProductionRuleSymbol {
    Token(Token),
    NonTerminal(NonTerminal),
    TokenClass(TokenClass),
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

        Self { grammar }
    }

    pub fn init_program_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Program,
            vec![vec![ProductionRuleSymbol::NonTerminal(NonTerminal::Statement)]],
        );
    }

    pub fn init_statement_production_rules(table: &mut GrammarTable) {
        table.insert(
            NonTerminal::Statement,
            vec![vec![
                ProductionRuleSymbol::NonTerminal(NonTerminal::AssignmentStatement),
                ProductionRuleSymbol::NonTerminal(NonTerminal::StatementPrime),
            ]],
        );
    }

    pub fn init_assignment_statement_production_rules(table: &mut GrammarTable) {
        table.insert(NonTerminal::AssignmentStatement, vec![vec![]]);
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
}
