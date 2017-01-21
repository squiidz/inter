extern crate inter;

use inter::lexer::Lexer;

fn main() {
    let mut lex = Lexer::new("let a = 5 + 3;");
    lex.next_token();
    println!("{}", lex);

}
