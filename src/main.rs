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
}

pub mod token {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Token {
        // special tokens
        Illegal,
        EOF,

        // keywords/identifiers
        Ident(String),
        Function,
        Let,

        // literal values
        Int(u32),

        // operators/special characters
        Assign,
        Plus,
        Minus,
        Comma,
        Dot,
        Semicolon,
        LParen, // (
        RParen, // )
        LBrace, // {
        RBrace, // }
        LBracket, // [
        RBracket, // ]
    }
}

pub mod lexer {
    use std::io;
    use super::token::Token;

    /// Lexer that excepts UTF-8 encoded source code (for simplicity).
    pub struct Lexer {
        /// input source code
        input: Vec<char>,
        /// position of the current character being processed in `input`
        position: usize,
        /// position of the next character being processed
        read_position: usize, 
        /// current character being processed
        chr: char
    }

    impl Lexer {
        pub fn new(source: String) -> Self {
            let input = source.chars().collect::<Vec<char>>();
            let mut slf = Self { 
                input,
                position: 0,
                read_position: 0, 
                chr: 0 as char
            };
            slf.read_char().expect("Invalid input/unexpected EOF");
            slf
        }

        // TODO: error handling -> only setting chr=0 isn't enough
        pub fn read_char(&mut self) -> io::Result<()> {
            if self.read_position >= self.input.len() {
                // We've reached the end of the input source
                self.chr = 0 as char;
                return io::Result::Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Reached end of input string!"));
            }
            self.chr = self.input[self.read_position];
            self.position = self.read_position;
            self.read_position += 1;

            Ok(())
        }

        pub fn determine_keyword(input: &str) -> Option<Token> {
            match input {
                "fn" => Some(Token::Function),
                "let" => Some(Token::Let),
                _ => None
            }
        }

        // wether the given character is valid in an identifier name
        fn is_valid(chr: char) -> bool {
            match chr {
                'A'..='Z' => true,
                'a'..='z' => true,
                '0'..='9' => true,
                '_' => true,
                _ => false
            }
        }

        fn is_ascii_whitespace(chr: char) -> bool {
            chr == '\t' || chr == ' '
        }

        pub fn parse_identifier(&mut self) -> io::Result<String> {
            let start = self.position;

            let mut i = 0;
            while i < self.input.len() {
                if !Self::is_valid(self.input[i]) || Self::is_ascii_whitespace(self.input[i]) {
                    break;
                }
                self.read_char()?;
                i += 1;
            }

            let result = self.input[start..self.position]
                .iter()
                .collect::<String>();

            Ok(result)
        }

        pub fn skip_whitespace(&mut self) -> io::Result<()> {
            while Self::is_ascii_whitespace(self.chr) {
                self.read_char()?;
            }
            Ok(())
        }

        pub fn next_token(&mut self) -> io::Result<Token> {
            self.skip_whitespace()?;
            let res = match self.chr {
                '0'..='9' => {
                    // safe to use unwrap
                    let num = self.chr.to_digit(10).unwrap();
                    Ok(Token::Int(num))
                },
                // parenthesis
                '(' => Ok(Token::LParen),
                ')' => Ok(Token::RParen),
                '{' => Ok(Token::LBrace),
                '}' => Ok(Token::RBrace),
                '[' => Ok(Token::LBracket),
                ']' => Ok(Token::RBracket),

                // operators
                '=' => Ok(Token::Assign),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                ',' => Ok(Token::Comma),
                ';' => Ok(Token::Semicolon),
                '.' => Ok(Token::Dot),

                // identifiers/keywords
                _   => {
                    let ident = self.parse_identifier()?;
                    // check if the string found is a keyword or should be treated as identifier
                    if let Some(kw) = Self::determine_keyword(&ident[..]) {
                        Ok(kw)
                    } else {
                        Ok(Token::Ident(ident))
                    }
                }
            };
            self.read_char()?;
            res
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
    }
}
