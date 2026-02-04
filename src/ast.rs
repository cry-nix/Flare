#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Ident(String),
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Var { name: String, value: Expr },
    Fn { name: String, body: Vec<Stmt> },
    Expr(Expr),
}
