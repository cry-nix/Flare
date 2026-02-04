use crate::lexer::Token;
use crate::ast::{Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn next(&mut self) -> &Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while self.peek() != &Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
        }

        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::Fn => self.parse_fn(),
            Token::Let => self.parse_let(),
            Token::Var => self.parse_var(),
            Token::Ident(_) => self.parse_expr_stmt(),
            _ => {
                self.next(); // skip unknown token for now
                None
            }
        }
    }

    fn parse_fn(&mut self) -> Option<Stmt> {
        self.next(); // consume 'fn'
        if let Token::Ident(name) = self.next().clone() {
            if self.next() != &Token::LParen { return None; }
            if self.next() != &Token::RParen { return None; }
            if self.next() != &Token::LBrace { return None; }

            let mut body = Vec::new();
            while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
                if let Some(stmt) = self.parse_stmt() {
                    body.push(stmt);
                }
            }

            self.next(); // consume RBrace
            Some(Stmt::Fn { name, body })
        } else {
            None
        }
    }

    fn parse_let(&mut self) -> Option<Stmt> {
        self.next(); // consume 'let'
        if let Token::Ident(name) = self.next().clone() {
            if self.next() != &Token::Eq { return None; }
            if let Some(value) = self.parse_expr() {
                return Some(Stmt::Let { name, value });
            }
        }
        None
    }

    fn parse_var(&mut self) -> Option<Stmt> {
        self.next(); // consume 'var'
        if let Token::Ident(name) = self.next().clone() {
            if self.next() != &Token::Eq { return None; }
            if let Some(value) = self.parse_expr() {
                return Some(Stmt::Var { name, value });
            }
        }
        None
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        if let Some(expr) = self.parse_expr() {
            Some(Stmt::Expr(expr))
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        match self.next() {
            Token::Number(n) => Some(Expr::Number(*n)),
            Token::String(s) => Some(Expr::String(s.clone())),
            Token::Ident(name) => {
                if self.peek() == &Token::LParen {
                    self.next(); // consume '('
                    let mut args = Vec::new();
                    while self.peek() != &Token::RParen && self.peek() != &Token::EOF {
                        if let Some(arg) = self.parse_expr() {
                            args.push(arg);
                        }
                    }
                    self.next(); // consume ')'
                    Some(Expr::Call { name: name.clone(), args })
                } else {
                    Some(Expr::Ident(name.clone()))
                }
            }
            _ => None,
        }
    }
}
