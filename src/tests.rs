#[cfg(test)]
mod lexer_tests {
    use crate::{token::Token, lexer::Lexer};

    #[test]
    fn basic_operators() {
        let source = String::from("{],.;=+");
        let expected = [
            Token::LBrace,
            Token::RBracket,
            Token::Comma,
            Token::Dot,
            Token::Semicolon,
            Token::Assign,
            Token::Plus
        ];
        let mut expected = expected.iter();

        let mut lexer = Lexer::new(source);
        while let Ok(t) = lexer.next_token() {
            let e = expected.next().unwrap();
            assert_eq!(t, *e);
        }
    }

    #[test]
    fn basic_source_code() {
        let source = r#"
let ten = 10;
let add = fn(x, y) {
x + y;
}
!-/*100<
        "#;
        let source = String::from(source);

        let expected = [
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Bang,
            Token::Minus,
            Token::Div,
            Token::Mul,
            Token::Int(100),
            Token::LessThan,
        ];
        let mut expected = expected.iter();

        let mut lexer = Lexer::new(source);
        while let Ok(t) = lexer.next_token() {
            let e = expected.next().unwrap();
            println!("L: {:?}\tR: {:?}", t, *e);
            assert_eq!(t, *e);
        }
    }

    #[test]
    fn if_else_return_booleans() {
        let source = String::from(r#"
if henlo else world return universe
        "#);
        let expected = [
            Token::If,
            Token::Ident(String::from("henlo")),
            Token::Else,
            Token::Ident(String::from("world")),
            Token::Return,
            Token::Ident(String::from("universe")),
        ];
        let mut expected = expected.iter();

        let mut lexer = Lexer::new(source);
        while let Ok(t) = lexer.next_token() {
            let e = expected.next().unwrap();
            println!("L: {:?}\tR: {:?}", t, *e);
            assert_eq!(t, *e);
        }
    }

    #[test]
    fn two_symbol_operators() {
        let source = String::from(r#"
let is_equal = 10 == 12;
        "#);
        let expected = [
            Token::Let,
            Token::Ident(String::from("is_equal")),
            Token::Assign,
            Token::Int(10),
            Token::Equals,
            Token::Int(12),
            Token::Semicolon,
        ];
        let mut expected = expected.iter();

        let mut lexer = Lexer::new(source);
        while let Ok(t) = lexer.next_token() {
            let e = expected.next().unwrap();
            println!("L: {:?}\tR: {:?}", t, *e);
            assert_eq!(t, *e);
        }
    }
}
