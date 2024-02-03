use lexer::lexer::Lexer;
use parser::parsers::RecursiveDescentParser;

fn main() {
    let lexer = Lexer::from_file("./test-files/main.cc").unwrap();
    let mut parser = RecursiveDescentParser::new(lexer);

    parser.parse().unwrap().print_tree()

    // while let Ok(info) = lexer.next() {
    //     println!("{:?}", info)
    // }
}
