use lexer::Token;

fn main() {
    let identifier_token = Token::IDENTIFIER("mistake".to_owned());
    let keyword_token = Token::KEYWORD("if".to_owned());
    println!(
        "Hello, Cool Compiler! Some examples: {} and {}",
        identifier_token, keyword_token
    );
}
