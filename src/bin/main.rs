extern crate inter;

use inter::lexer::Lexer;
use inter::parser::Parser;

fn main() {
    let code = "let a = 5 + 3;";
    let mut lex = Lexer::new(code);
    let mut parser = Parser::new(lex);

    parser.next_token();
    parser.next_token();
    parser.next_token();
    println!("{}", parser)
}
