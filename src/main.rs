use lexer::lexer::Lexer;
use parser::Parser;

fn main() {
    // let mut lexer = Lexer::from_file("./test-files/main.cc").unwrap();
    let lexer = Lexer::new("x -".to_owned());
    let mut parser = Parser::new(lexer);

    println!("{:?}", parser.parse());

    // while let Ok(info) = lexer.next() {
    //     println!("{:?}", info)
    // }
}
