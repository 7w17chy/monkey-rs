pub mod tests;
pub mod lexer;
pub mod token;

use lexer::Lexer;

fn main() {
    let input = String::from("{}[,;.]");
    let mut lexer = Lexer::new(input);
    while let Ok(c) = lexer.next_token() {
        println!("c: {:?}", c);
    }

    let input2 = String::from("}}}}}]");
    let mut lexer2 = Lexer::new(input2);
    while let Ok(c) = lexer2.next_token() {
        println!("c: {:?}", c);
    }

    let input3 = String::from(r#"
        ]\t"#);
    let mut lexer3 = Lexer::new(input3);
    while let Ok(c) = lexer3.next_token() {
        println!("c: {:?}", c);
    }
}
