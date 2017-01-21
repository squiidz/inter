use std::fmt::{self, Display, Formatter};
use token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[allow(dead_code)]
pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        return "".to_owned();
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }
        return out;
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        self.value.to_owned()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct LetStatement<'a> {
    token: Token,
    name: &'a Identifier,
    value: Option<Box<Expression>>,
}

impl<'a> Node for LetStatement<'a> {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&(self.token_literal() + " "));
        out.push_str(&self.name.string());
        out.push_str(" = ");
        match self.value.as_ref() {
            Some(v) => out.push_str(&v.string()),
            None => {}
        }
        out.push_str(";");
        out.to_owned()
    }
}

impl<'a> Statement for LetStatement<'a> {
    fn statement_node(&self) {}
}

impl<'a> Display for LetStatement<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct ReturnStatement {
    token: Token,
    return_value: Option<Box<Expression>>
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str(&(self.token_literal() + " "));
        match self.return_value.as_ref() {
            Some(v) => { out.push_str(&v.string()) },
            None => {},
        }

        out.push_str(";");
        out.to_owned()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct ExpressionStatement {
    token: Token,
    expression: Option<Box<Expression>>
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        match self.expression.as_ref() {
            Some(v) => { v.string() },
            None => { "".to_owned() },
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct IntegerLiteral {
    token: Token,
    value: i64
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct PrefixExpression {
    token: Token,
    operator: String,
    right: Expression
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str("(");
        out.push_str(&self.operator);
        out.push_str(&self.right.string());
        out.push_str(")");

        out.to_owned()
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct InfixExpression {
    token: Token,
    operator: String,
    left:  Box<Expression>,
    right: Box<Expression>
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str("(");
        out.push_str(&self.left.string());
        out.push_str(&format!(" {} " , self.operator));
        out.push_str(&self.right.string());
        out.push_str(")");

        out.to_owned()
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct Boolean {
    token: Token,
    value: bool
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {}
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct BlockStatement {
    token: Token,
    statements: Vec<Box<Statement>>
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(&s.string())
        }

        out.to_owned()
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct IfExpression {
    token: Token,
    condition:   Box<Expression>,
    consequence: Box<BlockStatement>,
    alternative: Option<Box<BlockStatement>>
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str("if");
        out.push_str(&self.condition.string());
        out.push_str(" ");
        out.push_str(&self.consequence.string());

        match self.alternative.as_ref() {
            Some(v) => {
                out.push_str("else ");
                out.push_str(&v.string());
            },
            None => {},
        }

        out.to_owned()
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct FunctionLiteral {
    token: Token,
    parameters: Vec<Box<Identifier>>,
    body: Box<BlockStatement>
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();

        for p in self.parameters.iter() {
            params.push(p.string());
        }
        out.push_str(&self.token_literal());
        out.push_str("(");
        out.push_str(&params.join(", "));
        out.push_str(") ");
        out.push_str(&self.body.string());

        out.to_owned()
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct CallExpression {
    token: Token,
    function: Box<Expression>,
    arguments: Vec<Box<Expression>>
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();

        for p in self.arguments.iter() {
            params.push(p.string());
        }

        out.push_str(&self.function.string());
        out.push_str("(");
        out.push_str(&params.join(", "));
        out.push_str(")");

        out.to_owned()
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct StringLiteral {
    token: Token,
    value: String
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl Expression for StringLiteral {
    fn expression_node(&self) {}
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[allow(dead_code)]
pub struct ArrayLiteral {
    token: Token,
    elements: Vec<Box<Expression>>
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut elements = Vec::new();

        for el in self.elements.iter() {
            elements.push(el.string());
        }

        out.push_str("[");
        out.push_str(&elements.join(", "));
        out.push_str("]");

        out.to_owned()
    }
}

impl Expression for ArrayLiteral {
    fn expression_node(&self) {}
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}
