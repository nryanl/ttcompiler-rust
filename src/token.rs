#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum TokenType {
    Unknown = -2,
    Eof = -1,
    Newline = 0,
    Number = 1,
    Ident = 2,
    String = 3,
    // Keywords.
    Label = 101,
    GoTo = 102,
    Print = 103,
    Input = 104,
    Let = 105,
    If = 106,
    Then = 107,
    EndIf = 108,
    While = 109,
    Repeat = 110,
    EndWhile = 111,
    // Operators
    Eq = 201,
    Plus = 202,
    Minus = 203,
    Asterisk = 204,
    Slash = 205,
    EqEq = 206,
    NotEq = 207,
    Lt = 208,
    LtEq = 209,
    Gt = 210,
    GtEq = 211,
}

// Rust will not automatically compare TokenType
impl PartialEq for TokenType {
    fn eq(&self, rhs: &TokenType) -> bool {
        let l = *self as i32;
        let r = *rhs as i32;
        l == r
    }
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self {
        Token { text, kind }
    }

    pub fn check_if_keyword(text: &str) -> TokenType {
        match text {
            "LABEL" => TokenType::Label,
            "GOTO" => TokenType::GoTo,
            "PRINT" => TokenType::Print,
            "INPUT" => TokenType::Input,
            "LET" => TokenType::Let,
            "IF" => TokenType::If,
            "THEN" => TokenType::Then,
            "ENDIF" => TokenType::EndIf,
            "WHILE" => TokenType::While,
            "REPEAT" => TokenType::Repeat,
            "ENDWHILE" => TokenType::EndWhile,
            _ => TokenType::Unknown,
        }


    }
}