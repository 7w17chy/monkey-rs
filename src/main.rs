pub mod tests;
pub mod lexer;
pub mod token;

//use std::env;

use lexer::Lexer;
use token::Token;

fn main() {
    let source = String::from(r#"
let is_equal = 10;;
    "#);

    let expected = [
        Token::Let,
        Token::Ident(String::from("is_equal")),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::EOF,
    ];
    let mut expected = expected.iter();

    let mut lexer = Lexer::new(source);
    while let Some(t) = lexer.next_token() {
        let e = expected.next().unwrap();
        eprintln!("L: {:?}\tR: {:?}", t, *e);
        assert_eq!(t, *e);
    }
}
