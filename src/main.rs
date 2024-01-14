use lexer::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::from_file("./test-files/main.cc").unwrap();

    while let Ok(info) = lexer.next() {
        println!("{:?}", info)
    }
}
