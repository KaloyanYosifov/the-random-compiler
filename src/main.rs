use lexer::Token;

fn main() {
    let identifier_token = Token::Identifier("mistake".to_owned());
    let keyword_token = Token::Keyword("if".to_owned());
    println!(
        "Hello, Cool Compiler! Some examples: {} and {}",
        identifier_token, keyword_token
    );
}
