use token::Token;

trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<Statement>>
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        return "".to_owned()
    }

    fn string(&self) -> String {
        let mut out: String = "".to_owned();
        return out
    }
}
