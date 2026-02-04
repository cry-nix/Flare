#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Module,
    Safe,
    Fn,
    Let,
    Var,
    Ident(String),
    Number(i64),
    String(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Eq,
    EOF,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }

            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '=' => { tokens.push(Token::Eq); chars.next(); }

            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' { break; }
                    s.push(c);
                    chars.next();
                }
                chars.next();
                tokens.push(Token::String(s));
            }

            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if !c.is_digit(10) { break; }
                    num.push(c);
                    chars.next();
                }
                tokens.push(Token::Number(num.parse::<i64>().unwrap()));
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut id = String::new();
                while let Some(&c) = chars.peek() {
                    if !c.is_alphanumeric() && c != '_' { break; }
                    id.push(c);
                    chars.next();
                }
                match id.as_str() {
                    "module" => tokens.push(Token::Module),
                    "safe" => tokens.push(Token::Safe),
                    "fn" => tokens.push(Token::Fn),
                    "let" => tokens.push(Token::Let),
                    "var" => tokens.push(Token::Var),
                    _ => tokens.push(Token::Ident(id)),
                }
            }

            _ => {
                panic!("Unknown character in source: '{}'", ch);
            }
        }
    }

    tokens.push(Token::EOF);
    tokens
}
