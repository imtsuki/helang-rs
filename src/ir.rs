#[derive(Debug)]
pub enum Literal {
    Number(i64),
    Array(Vec<i64>),
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Expr {
    Lit(Literal),
    Index(Ident, Literal),
    Ident(Ident),
}

#[derive(Debug)]
pub enum Stmt {
    Decl(Ident, Literal),
    Assign(Expr, Literal),
    Print(Expr),
    Test5G,
}
