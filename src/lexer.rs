use crate::token::Token;

use std::iter::Iterator;

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

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
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
            read_position: 1, 
            chr: 0 as char,
        };
        slf.chr = slf.input[slf.position];
        slf
    }

    /// Advance the `pointers` further into the input string. 
    fn advance(&mut self) -> Option<()> {
        self.position = self.read_position;
        self.read_position += 1;

        if self.position >= self.input.len() {
            // we've reached the end of the input string
            eprintln!("advance: reached end of the input string");
            self.chr = '\0';
            None
        } else {
            self.chr = self.input[self.position];
            eprintln!("advance: advancing to input[{}] = '{}'", 
                self.position, self.chr);
            Some(())
        }
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

    fn peek(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_position])
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

    #[inline]
    fn is_ascii_whitespace(chr: char) -> bool {
        chr == '\t' || chr == ' ' || chr == '\n'
    }

    fn skip_whitespace(&mut self) {
        while Self::is_ascii_whitespace(self.chr) {
            eprintln!("skipping whitespace at input[{}]", self.position);
            // we've reached the end of the input string
            if let None = self.advance() {
                break;
            }
        }
    }

    /// Parse an identifier which may be an identfier in the sense of a name or keyword
    /// (seperation between the two is done in another, seperate step).
    fn parse_identifier(&mut self) -> Option<String> {
        self.skip_whitespace();

        let start = self.position;
        let mut i = start;

        while let Some(chr) = self.peek() {
            //eprintln!("parse_identifier: input[{}] = '{}'", i, self.input[i]);
            if !Self::is_valid(chr) || Self::is_ascii_whitespace(chr) {
                eprintln!("parse_identifier: breaking: input[{}] = '{}'", i, self.input[i]);
                break;
            }
            self.advance()?;
            i += 1;
        }

        // NOTE: i+1: upper slice bound is exclusive!
        let result = self.input[start..i+1]
            .iter()
            .collect::<String>();

        if (i - start) == 0 {
            // empty identifier
            None
        } else {
            Some(result)
        }
    }

    /// parse integer from input sources
    fn parse_integer(&mut self) -> Option<u32> {
        let is_digit = |c: char| match c {
            '0'..='9' => true,
            _ => false,
        };

        let mut nums = Vec::with_capacity(10);
        eprintln!("reading integer starting at: input[{}] = '{}'", 
            self.position, self.input[self.position]);
        while let Some(chr) = self.peek() {
            if !is_digit(chr) {
                break;
            }
            eprintln!("input[{}] = '{}'", self.position, self.input[self.position]);
            nums.push(self.chr.to_digit(10).unwrap());
            self.advance()?;
        }

        // every digit gets multiplied by 10 raised by it's index and then added
        // e.g. :
        // [1, 2, 3] = [100, 20, 3] = 123
        let res = nums
            .iter()
            .enumerate()
            .map(|(i, v)| v * 10u32.pow((i+1) as u32))
            .sum::<u32>();

        Some(res)
    }

    /// Advance by `amount` characters into the input string and return the amount skipped.
    pub fn skip_chars(&mut self, amount: usize) -> usize {
        if (self.position + amount) > self.input.len() {
            let amount_skipped = self.input.len() - self.position;
            // skip until the end of the input string.
            // The caller can find out if the end has been reached when they compare the
            // input and output numbers (the diff should =0)
            self.position = self.input.len();
            self.read_position = self.position + 1;
            self.chr = '\0';
            eprintln!("skip_chars: skipped until the end of the input string");
            amount_skipped
        } else {
            // update fields accordingly
            self.position = self.position + amount;
            self.read_position = self.position + 1;
            self.chr = self.input[self.position];
            eprintln!("skip_chars: skipped {} chars, input[{}] = '{}'",
                amount, self.position, self.chr);
            amount
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let res = match self.chr {
            '0'..='9' => self.parse_integer().map(Token::Int),
            '\0' => None,
            // parenthesis
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),

            // operators
            '=' => match self.peek() {
                Some('=') => {
                    // doesn't matter if we skip to the end
                    _ = self.skip_chars(1);
                    if self.chr == '=' {
                        eprintln!("didn't skip enough characters!");
                    }
                    Some(Token::Equals)
                },
                _ => Some(Token::Assign),
            },
            '!' => match self.peek() {
                Some('=') => {
                    // doesn't matter if we skip to the end
                    _ = self.skip_chars(1);
                    if self.chr == '=' {
                        eprintln!("didn't skip enough characters!");
                    }
                    Some(Token::DoesntEqual)
                },
                _ => Some(Token::Bang),
            },

            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semicolon),
            '.' => Some(Token::Dot),

            // identifiers/keywords
            _   => {
                eprintln!("trying to read identifier beginning with: '{}'", self.chr);
                let ident = match self.parse_identifier() {
                    Some(i) => i,
                    None => {
                        eprintln!("called parse_identifier on an invalid input");
                        panic!("neither operator, keyword or valid token");
                    }
                };

                // check if the string found is a keyword or should be treated as identifier
                match Self::determine_keyword(&ident[..]) {
                    Some(kw) => Some(kw),
                    None => Some(Token::Ident(ident)),
                }
            }
        };
        // return value is handled when `next_token` is called again
        _ = self.advance();
        res
    }
}
