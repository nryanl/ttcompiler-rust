use std::collections::HashSet;

use crate::{lexer::Lexer, token::{TokenType, Token}, emitter::Emitter};


pub struct Parser<'a> {
    lexer: Lexer,
    emitter: &'a mut Emitter,
    cur_token: Token,
    peek_token: Token,
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer, emitter: &'a mut Emitter) -> Self {
        let mut s = Self {
            lexer,
            emitter,
            cur_token: Token::default(),
            peek_token: Token::default(),
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_gotoed: HashSet::new(),
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
        // Require at least one newline.
        self.match_token(TokenType::Newline);

        // Allow extra newlines
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }

    /// program ::= {statement}
    pub fn program(&mut self) {
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main(void){");

        // Since some newlines are required in our grammar, need to skip the excess.
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }

        // Parse all the statements in the program.
        while !self.check_token(TokenType::Eof) {
            self.statement();
        }

        // Wrap things up.
        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");

        // Check that each label referenced in a GOTO is declared
        self.labels_gotoed.iter()
        .filter(|label| !self.labels_declared.contains(label.as_str()))
        .for_each(|label| {
            self.abort(format!("Attempting to GOTO undeclared label: {}", label).as_str());
        });
    }

    /// One of the following statements...
    pub fn statement(&mut self) {
        // Check the first otken to see what kind of statement this is.

        match self.cur_token.kind {
            TokenType::Print => {
                self.next_token();

                if self.check_token(TokenType::String) {
                    // Simple string, so print it.
                    self.emitter.emit_line(format!("printf(\"{}\\n\");", self.cur_token.text).as_str());
                    self.next_token();
                } else {
                    // Expect an expression
                    self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                    self.expression();
                    self.emitter.emit_line("));");
                }
            },
            TokenType::If => {
                self.next_token();
                self.emitter.emit("if(");
                self.comparison();

                self.match_token(TokenType::Then);
                self.nl();
                self.emitter.emit_line("){");

                // Zero of more statements in the body
                while !self.check_token(TokenType::EndIf) {
                    self.statement();
                }
                
                self.match_token(TokenType::EndIf);
                self.emitter.emit_line("}");
            },
            TokenType::While => {
                self.next_token();
                self.emitter.emit("while(");
                self.comparison();

                self.match_token(TokenType::Repeat);
                self.nl();
                self.emitter.emit_line("){");

                // Zero or more statements in the loop body.
                while !self.check_token(TokenType::EndWhile) {
                    self.statement();
                }

                self.match_token(TokenType::EndWhile);
                self.emitter.emit_line("}");
            },
            TokenType::Label => {
                self.next_token();

                if self.labels_declared.contains(&self.cur_token.text) {
                    self.abort(format!("Label already exists: {}", self.cur_token.text).as_str());
                }
                self.labels_declared.insert(self.cur_token.text.clone());

                self.emitter.emit_line(format!("{}:", self.cur_token.text).as_str());
                self.match_token(TokenType::Ident);
            },
            TokenType::GoTo => {
                self.next_token();
                self.labels_gotoed.insert(self.cur_token.text.clone());
                self.emitter.emit_line(format!("goto {};", self.cur_token.text).as_str());
                self.match_token(TokenType::Ident);
            },
            TokenType::Let => {
                self.next_token();

                // Check if ident exists in symbol table. If not, declare it.
                if !self.symbols.contains(&self.cur_token.text) {
                    self.symbols.insert(self.cur_token.text.clone());
                    self.emitter.header_line(format!("float {};", self.cur_token.text).as_str());
                }

                self.emitter.emit(format!("{} = ", self.cur_token.text).as_str());
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);

                self.expression();
                self.emitter.emit_line(";");
            },
            TokenType::Input => {
                self.next_token();

                // If variable doesn't already exist, declare it.
                if !self.symbols.contains(&self.cur_token.text) {
                    self.symbols.insert(self.cur_token.text.clone());
                    self.emitter.header_line(format!("float {};", self.cur_token.text).as_str());
                }

                // Emit scanf but also validate the input. If invalid, set the variable to 0 and clear the input.
                self.emitter.emit_line(format!("if(0 == scanf(\"%f\", &{})) {{", self.cur_token.text).as_str());
                self.emitter.emit_line(format!("{} = 0;", self.cur_token.text).as_str());
                self.emitter.emit("scanf(\"%");
                self.emitter.emit_line("*s\");");
                self.emitter.emit_line("}");
                self.match_token(TokenType::Ident);
            },
            _ => {
                self.abort(format!("Invalid statement at {}", self.cur_token.text).as_str());
            }
        }

        self.nl()
    }

    pub fn expression(&mut self) {
        self.term();
        // Can have 0 or more +/- and expressions
        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.term();
        }
    }

    pub fn term(&mut self) {
        self.unary();
        // Can have 0 or more *// and expressions.
        while self.check_token(TokenType::Asterisk) || self.check_token(TokenType::Slash) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.unary();
        }
    }

    pub fn unary(&mut self) {
        // Optional unary +/-
        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        }
        self.primary();
    }

    pub fn primary(&mut self) {
        match self.cur_token.kind {
            TokenType::Number => {
                self.emitter.emit(&self.cur_token.text);
                self.next_token();
            },
            TokenType::Ident => {
                if !self.symbols.contains(&self.cur_token.text) {
                    self.abort(format!("Referencing variable before assignment: {}", self.cur_token.text).as_str());
                }

                self.emitter.emit(&self.cur_token.text);
                self.next_token();
            }
            _ => {
                self.abort(format!("Unexpected token at {}", self.cur_token.text).as_str());
            }
        }
    }

    pub fn comparison(&mut self) {
        self.expression();

        // Must be at least one comparison operator and another expression.
        if self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression();
        } else {
            self.abort(format!("Expected comparison operator at: {}", self.cur_token.text).as_str());
        }

        // Can have 0 or more comparison operator and expressions.
        while self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression();
        }
    }
}
