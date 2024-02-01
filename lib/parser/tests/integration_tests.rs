use lexer::lexer::Lexer;
use parser::Parser;

#[test]
fn test_it_generates_a_correct_parse_tree() {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("test-files/main.cc");

    let lexer = Lexer::from_file(current_dir.to_str().unwrap()).unwrap();
    let mut parser = Parser::new(lexer);

    insta::assert_debug_snapshot!(parser.parse().unwrap());
}
