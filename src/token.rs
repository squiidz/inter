use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,
    STRING,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    EQ,
    NOT_EQ,

    COMMA, // ,
    SEMICOLON, // ;
    LBRACKET, // [
    RBRACKET, // ]
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    LT,
    GT,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

lazy_static! {
    static ref keywords: HashMap<&'static str, TokenType> = {
        let mut hm = HashMap::new();
        hm.insert("fn", TokenType::FUNCTION);
        hm.insert("let", TokenType::LET);
        hm.insert("true", TokenType::TRUE);
        hm.insert("false", TokenType::FALSE);
        hm.insert("if", TokenType::IF);
        hm.insert("else", TokenType::ELSE);
        hm.insert("return", TokenType::RETURN);
        hm
    };
}

#[derive(Debug)]
pub struct Token {
    pub token: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: char) -> Token {
        Token {
            token: token_type,
            literal: literal.to_string(),
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        match keywords.get(ident) {
            Some(&t) => { t.clone() },
            None => { TokenType::IDENT },
        }
    }
}