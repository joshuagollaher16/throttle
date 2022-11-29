use crate::lexer::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BooleanOp {
    And,
    Not,
    Or,
    GT,
    LT,
    Eq,
    NGT,
    NLTm,
    NEq,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i32),
    Float(f32),
    QuotedStr(String),
    Name(String),
}

#[derive(Clone, Debug)]
pub enum Node {
    IfStatement {
        test: Box<Node>,
        body: Vec<Node>,
    },
    ReturnStatement(Box<Node>),
    Expression {
        start: Box<Node>,
        body: Vec<(BooleanOp, Node)>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Node>,
    },
    VariableAssignment {
        lhs: String,
        rhs: Box<Node>,
    },
    Literal(LiteralValue),
}


pub struct VM {}

impl VM {
    pub fn build(tokens: Vec<Token>) -> Self {
        let index = 0;

        let parse_expr = |tokens: &[Token], until: TokenKind| {};

        Self {}
    }
}