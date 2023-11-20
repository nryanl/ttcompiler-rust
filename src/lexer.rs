use crate::token::{TokenType, Token, self};

pub struct Lexer {
    source: String,
    pub cur_char: char,
    cur_pos: i32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut s = Self {
            source: source.clone() + "\n",
            cur_char: ' ',
            cur_pos: -1
        };
        s.next_char();
        s
    }

    /// Process the next character.
    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        let p = self.cur_pos as usize;
        if p >= self.source.len() {
            self.cur_char = '\0'; // EOF
        } else {
            self.cur_char = self.source.as_bytes()[p] as char;
        }
    }

    /// Return the lookahead character.
    pub fn peek(&self) -> char {
        let p = self.cur_pos as usize + 1;
        if p >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[p] as char
        }
    }

    /// Invalid token found, print error message and exit.
    pub fn abort(&self, message: String) {
        panic!("{message}");
    }

    /// Skip whitespace except newlines, 
    /// which we will use to indicate the end of a statement.
    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    /// Skip comments in the code.
    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }

    /// Return the next token.
    pub fn get_token(&mut self) -> Token {
        // Check the first character of this token to see if 
        // we can decide what it is. If it is a multiple 
        // character operator (e.g., !=), number, identifier, 
        // or keyword then we will process the rest.
        self.skip_whitespace();
        self.skip_comment();

        let mut token_text = String::from(self.cur_char);

        let token_type = match self.cur_char {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            '"' => {
                // Get characters between quotations.
                self.next_char();
                let start_pos = self.cur_pos;

                while self.cur_char != '"' {
                    match self.cur_char {
                        '\r' | '\n' | '\t' | '\\' | '%' => {
                            self.abort("Illegal character in string.".into());
                        }
                        _ => {
                            self.next_char();
                        }
                    }
                }
                token_text = self.source.get(start_pos as usize..self.cur_pos as usize).unwrap().to_string();
                TokenType::String
            },
            '!' => {
                if self.peek() == '=' {
                    self.next_char();
                    token_text.push(self.cur_char);
                    TokenType::LtEq
                } else {
                    self.abort(format!("Expected !=, got !{}", self.peek()));
                    TokenType::Unknown
                }
            },
            '=' => {
                if self.peek() == '=' {
                    self.next_char();
                    token_text.push(self.cur_char);
                    TokenType::EqEq
                } else {
                    TokenType::Eq
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.next_char();
                    token_text.push(self.cur_char);
                    TokenType::GtEq
                } else {
                    TokenType::Gt
                }
            },
            '<' => {
                if self.peek() == '=' {
                    self.next_char();
                    token_text.push(self.cur_char);
                    TokenType::LtEq
                } else {
                    TokenType::Lt
                }
            },
            '0'..='9' => {
                // Leading character is a digit, so this must be a number.
                // Get all consecutive digits and decimal if there is one.
                let start_pos = self.cur_pos;
                while self.peek().is_ascii_digit() {
                    self.next_char();
                }
                if self.peek() == '.' {
                    self.next_char();
                    if !self.peek().is_ascii_digit() {
                        // Error
                        self.abort("Illegal character in number.".into())
                    }
                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }
                }

                token_text = self.source.get(start_pos as usize..self.cur_pos as usize + 1).unwrap().to_string();
                TokenType::Number
            },
            'a'..='z' | 'A'..='Z' => {
                // Leading character is a letter, so this must be an identifier or a keyword
                // Get all consecutive alpha numeric characters
                let start_pos = self.cur_pos;
                while self.peek().is_alphanumeric() {
                    self.next_char();
                }

                // Check if the token is in the list of keywords
                token_text = self.source.get(start_pos as usize..self.cur_pos as usize + 1).unwrap().to_string();
                let key_word = Token::check_if_keyword(&token_text);
                if key_word == TokenType::Unknown {
                    TokenType::Ident
                } else {
                    key_word
                }
            }
            '\n' => TokenType::Newline,
            '\0' => TokenType::Eof,
            _ => panic!()
        };
        let token = Token::new(token_text, token_type);

        self.next_char();
        token
    }
}