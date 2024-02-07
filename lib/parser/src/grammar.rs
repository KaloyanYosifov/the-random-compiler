use lexer::token::{Token, TokenClass, KEYWORDS};
use std::collections::HashMap;
use strum::{Display, EnumString};

#[derive(PartialEq, Eq, Hash, EnumString, Display)]
pub enum Terminal {
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
    Terminal(Terminal),
    TokenClass(TokenClass),
}

pub type ProductionRule = Vec<ProductionRuleSymbol>;
pub type ProductionRules = Vec<ProductionRule>;
pub type GrammarTable = HashMap<Terminal, ProductionRules>;

pub struct Grammar {
    grammar: HashMap<Terminal, ProductionRules>,
}

impl Grammar {
    pub fn new() -> Self {
        let mut grammar = HashMap::new();

        Self::init_program_production_rules(&mut grammar);

        Self { grammar }
    }

    pub fn init_program_production_rules(table: &mut GrammarTable) {
        table.insert(
            Terminal::Program,
            vec![vec![ProductionRuleSymbol::Terminal(Terminal::Statement)]],
        );
    }

    pub fn init_statement_production_rules(table: &mut GrammarTable) {
        table.insert(
            Terminal::Statement,
            vec![vec![
                ProductionRuleSymbol::Terminal(Terminal::AssignmentStatement),
                ProductionRuleSymbol::Terminal(Terminal::StatementPrime),
            ]],
        );
    }

    pub fn init_assignment_statement_production_rules(table: &mut GrammarTable) {
        table.insert(Terminal::AssignmentStatement, vec![vec![]]);
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

        table.insert(Terminal::Keyword, production_rules);
    }
}
