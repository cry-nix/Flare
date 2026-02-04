mod lexer;
mod parser;
mod ast;

use lexer::lex;
use parser::Parser;

fn main() {
    let code = r#"
    fn main() {
        let x = 0
        print("Hello Flare")
    }
    "#;

    let tokens = lex(code);
    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);
}
