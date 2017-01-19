extern crate inter;

use inter::lexer::Lexer;

fn main() {
    let lex = Lexer::new("some text to lex");
    println!("{:?}", lex);

}
