#[cfg(test)]
mod lexer_tests {
    use crate::{token::Token, lexer::Lexer};

    #[test]
    fn single_character_identifiers() {
        let source = String::from(r#"
let y = f(x);
        "#);
        let expected = [
            Token::Let,
            Token::Ident("y".to_string()),
            Token::Assign,
            Token::Ident("f".to_string()),
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::EOF,
        ];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            eprintln!("t = {:?}", t);
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR:{:?}", t, *e);
            assert_eq!(t, *e);
        }
    }

    #[test]
    fn operator_after() {
        let source = String::from("henlo! ==!= 10; 10= 10== 10!=");
        let expected = [
            Token::Ident("henlo".to_string()),
            Token::Bang,
            Token::Equals,
            Token::DoesntEqual,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::Assign,
            Token::Int(10),
            Token::Equals,
            Token::Int(10),
            Token::DoesntEqual,
            Token::EOF,
        ];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            eprintln!("t = {:?}", t);
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR:{:?}", t, *e);
            assert_eq!(t, *e);
        }
    }

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
            Token::Plus,
            Token::EOF
        ];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            eprintln!("t = {:?}", t);
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR:{:?}", t, *e);
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
            Token::EOF,
        ];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR: {:?}", t, *e);
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
            Token::EOF,
        ];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR: {:?}", t, *e);
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
            Token::EOF,
        ];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR: {:?}", t, *e);
            assert_eq!(t, *e);
        }
    }

    #[test]
    fn two_symbol_operators2() {
        let source = String::from("== !=");
        let expected = vec![Token::Equals, Token::DoesntEqual, Token::EOF];
        let mut expected = expected.iter();

        let lexer = Lexer::new(source);
        for t in lexer {
            let e = expected.next().unwrap();
            eprintln!("L: {:?}\tR: {:?}", t, *e);
            assert_eq!(t, *e);
        }

    }
}
