use std::io;
use crate::token::Token;

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
        let source = source
            .replace("\n", " ")
            .replace("\r\n", " ");

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

    fn read_char(&mut self) -> io::Result<()> {
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

    fn determine_keyword(input: &str) -> Option<Token> {
        match input {
            "fn" => Some(Token::Function),
            "let" => Some(Token::Let),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "return" => Some(Token::Return),
            "true" => Some(Token::Boolean(true)),
            "false" => Some(Token::Boolean(false)),
            _ => None,
        }
    }

    fn peek(&mut self) -> io::Result<char> {
        if self.read_position >= self.input.len() {
            io::Result::Err(io::Error::new(io::ErrorKind::Other,
                            String::from("Would reach beyound the end of the input!")))
        } else {
            Ok(self.input[self.read_position])
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
        chr == '\t' || chr == ' ' || chr == '\n'
    }

    fn skip_whitespace(&mut self) -> io::Result<()> {
        while Self::is_ascii_whitespace(self.chr) {
            self.read_char()?;
        }
        Ok(())
    }

    fn parse_identifier(&mut self) -> io::Result<String> {
        self.skip_whitespace()?;
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

        if (self.position - start) == 0 {
            return io::Result::Err(io::Error::new(io::ErrorKind::Other,
                                   String::from("Empty identifier")));
        }
        Ok(result)
    }

    /// parse integer from input sources
    fn parse_integer(&mut self) -> io::Result<u32> {
        let is_digit = |c: char| match c {
            '0'..='9' => true,
            _ => false,
        };

        let mut nums = Vec::with_capacity(10);
        while is_digit(self.chr) {
            nums.push(self.chr.to_digit(10).unwrap());
            self.read_char()?;
        }

        // every digit gets multiplied by 10 raised by it's index and then added
        // e.g. :
        // [1, 2, 3] = [100, 20, 3] = 123
        let res = nums
            .iter()
            .enumerate()
            .map(|(i, v)| v * 10u32.pow(i as u32))
            .sum::<u32>();

        Ok(res)
    }

    pub fn skip_chars(&mut self, amount: usize) -> io::Result<()> {
        if (self.position + amount) > self.input.len() {
            return io::Result::Err(io::Error::new(io::ErrorKind::Other,
                                   String::from(
                                       format!(
                                           "Skipping {} chars would exceed the length of the input!",
                                           amount
                                        )
                                   )));

        }
        Ok(())
    }

    pub fn next_token(&mut self) -> io::Result<Token> {
        self.skip_whitespace()?;
        let res = match self.chr {
            '0'..='9' => {
                let num = self.parse_integer()?;
                Ok(Token::Int(num))
            },
            '\0' => Ok(Token::EOF),
            // parenthesis
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),

            // operators
            '=' => match self.peek() {
                Ok('!') => {
                    self.skip_chars(2)?;
                    Ok(Token::DoesntEqual)
                },
                Ok('=') => {
                    self.skip_chars(2)?;
                    Ok(Token::Equals)
                },
                _       => Ok(Token::Assign),
            },

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
