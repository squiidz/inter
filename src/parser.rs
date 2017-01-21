use std::collections::HashMap;

use lexer::Lexer;
use token::{Token, TokenType};
use ast;

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
enum PrecedenceType {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}


lazy_static! {
    static ref PRECEDENCES: HashMap<TokenType, PrecedenceType> = {
        let mut hm = HashMap::new();
        hm.insert(TokenType::EQ, PrecedenceType::EQUALS);
        hm.insert(TokenType::NOT_EQ, PrecedenceType::EQUALS);
        hm.insert(TokenType::LT, PrecedenceType::LESSGREATER);
        hm.insert(TokenType::GT, PrecedenceType::LESSGREATER);
        hm.insert(TokenType::PLUS, PrecedenceType::SUM);
        hm.insert(TokenType::MINUS, PrecedenceType::SUM);
        hm.insert(TokenType::SLASH, PrecedenceType::PRODUCT);
        hm.insert(TokenType::ASTERISK, PrecedenceType::PRODUCT);
        hm.insert(TokenType::LPAREN, PrecedenceType::CALL);
        hm
    };
}

type prefixParseFn = fn() -> ast::Expression;
type infixParseFn = fn(ast::Expression) -> ast::Expression;

struct Parser {
    lex: Box<Lexer>,
    cur_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, prefixParseFn>,
    infix_parse_fns: HashMap<TokenType, infixParseFn>,
    errors: Vec<String>
}

impl Parser {
    fn new(l: &mut Lexer) -> Parser {
        let mut parser = Parser{
            lex: Box::new(l.clone()),
            cur_token: Token{token: TokenType::EOF, literal: String::new()},
            peek_token: Token{token: TokenType::EOF, literal: String::new()},
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
            errors: Vec::new(),
        };

        parser.init_prefix();
        parser.init_infix();
        parser
    }

    fn init_prefix(&mut self) {
        unimplemented!()
        //self.register_prefix(TokenType::IDENT, self.parseIdentifier);
    }

    fn init_infix(&mut self) {
        unimplemented!()
    }

    fn cur_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.cur_token.token) {
            Some(p) => { p.clone() },
            None => { PrecedenceType::LOWEST }
        }
    }

    fn peek_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.peek_token.token) {
            Some(p) => { p.clone() },
            None => { PrecedenceType::LOWEST }
        }
    }

    fn register_prefix(&mut self, tt: TokenType, func: prefixParseFn) {
        self.prefix_parse_fns.insert(tt, func);
    }

    fn register_infix(&mut self, tt: TokenType, func: infixParseFn) {
        self.infix_parse_fns.insert(tt, func);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    fn parse_program(&self) -> ast::Program {
        unimplemented!()
    }
}
