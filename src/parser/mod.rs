struct Program {
    statements: Vec<Statement>,
}

pub enum Statement {

}

pub enum Expr {
    Unary(Box<Operator>, Box<Expr>),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Literal(Literal)
}
