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
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }
        return out
    }
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String { self.token.literal.to_owned() }
    fn string(&self) -> String {
        self.value.to_owned()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

struct LetStatement<'a> {
    token: Token,
    name: &'a Identifier,
    value: Expression,
}
