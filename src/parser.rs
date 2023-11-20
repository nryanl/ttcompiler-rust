use crate::{lexer::Lexer, token::{TokenType, Token}};


pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut s = Self {
            lexer,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };
        s.next_token();
        s.next_token();
        s
    }

    /// Return true if the current token matches.
    pub fn check_token(&self, kind: TokenType) -> bool {
        kind == self.cur_token.kind
    }

    /// Return true if the next token matches.
    pub fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.peek_token.kind
    }

    /// Try to match current token. If not, error. Advances the current token.
    pub fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind) {
            self.abort(format!("Expected {:?}, got {:?}", kind, self.cur_token.kind).as_str());
        }
        self.next_token();
    }

    /// Advances the current token.
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    pub fn abort(&self, message: &str) {
        panic!("{message}");
    }
    
    pub fn is_comparison_operator(&self) -> bool {
        self.check_token(TokenType::Gt)   || 
        self.check_token(TokenType::GtEq) ||
        self.check_token(TokenType::Lt)   ||
        self.check_token(TokenType::LtEq) ||
        self.check_token(TokenType::EqEq) ||
        self.check_token(TokenType::NotEq)
    }

    /// nl ::= '\n'+
    pub fn nl(&mut self) {
        println!("NEWLINE");

        // Require at least one newline.
        self.match_token(TokenType::Newline);

        // Allow extra newlines
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }

    /// program ::= {statement}
    pub fn program(&mut self) {
        println!("PROGRAM");

        // Since some newlines are required in our grammar, need to skip the excess.
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }

        // Parse all the statements in the program.
        while !self.check_token(TokenType::Eof) {
            self.statement();
        }
    }

    /// One of the following statements...
    pub fn statement(&mut self) {
        // Check the first otken to see what kind of statement this is.

        match self.cur_token.kind {
            TokenType::Print => {
                println!("STATEMENT-PRINT");
                self.next_token();

                if self.check_token(TokenType::String) {
                    // Simple string.
                    self.next_token();
                } else {
                    // Expect an expression
                    self.expression();
                }
            },
            TokenType::If => {
                println!("STATEMENT-IF");
                self.next_token();
                self.comparison();

                self.match_token(TokenType::Then);
                self.nl();

                // Zero of more statements in the body
                while !self.check_token(TokenType::EndIf) {
                    self.statement();
                }
                
                self.match_token(TokenType::EndIf);
            },
            TokenType::While => {
                println!("STATEMENT-WHILE");
                self.next_token();
                self.comparison();

                self.match_token(TokenType::Repeat);
                self.nl();

                // Zero or more statements in the loop body.
                while !self.check_token(TokenType::EndWhile) {
                    self.statement();
                }

                self.match_token(TokenType::EndWhile);
            },
            TokenType::Label => {
                println!("STATEMENT-LABEL");
                self.next_token();
                self.match_token(TokenType::Ident);
            },
            TokenType::GoTo => {
                println!("STATEMENT-GOTO");
                self.next_token();
                self.match_token(TokenType::Ident);
            },
            TokenType::Let => {
                println!("STATEMENT-LET");
                self.next_token();
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);
                self.expression();
            },
            TokenType::Input => {
                println!("STATEMENT-INPUT");
                self.next_token();
                self.match_token(TokenType::Ident);
            },
            _ => {
                self.abort(format!("Invalid statement at {}", self.cur_token.text).as_str());
            }
        }

        self.nl()
    }

    pub fn expression(&mut self) {
        println!("EXPRESSION");

        self.term();
        // Can have 0 or more +/- and expressions
        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.next_token();
            self.term();
        }
    }

    pub fn term(&mut self) {
        println!("TERM");

        self.unary();
        // Can have 0 or more *// and expressions.
        while self.check_token(TokenType::Asterisk) || self.check_token(TokenType::Slash) {
            self.next_token();
            self.unary();
        }
    }

    pub fn unary(&mut self) {
        println!("UNARY");

        // Optional unary +/-
        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.next_token();
        }
        self.primary();
    }

    pub fn primary(&mut self) {
        println!("PRIMARY ({})", self.cur_token.text);

        if self.check_token(TokenType::Number) || self.check_token(TokenType::Ident) {
            self.next_token();
        }
        else {
            // Error
            self.abort(format!("Unexpected token at {}", self.cur_token.text).as_str());
        }
    }

    pub fn comparison(&mut self) {
        println!("COMPARISON");

        self.expression();
        // Must be at least one comparison operator and another expression.
        if self.is_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            self.abort(format!("Expected comparison operator at: {}", self.cur_token.text).as_str());
        }

        // Can have 0 or more comparison operator and expressions.
        while self.is_comparison_operator() {
            self.next_token();
            self.expression();
        }
    }
}
