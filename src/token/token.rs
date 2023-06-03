
//struct Token {
//    Type: TokenType,
//    Literal: u8,
//}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Token{
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}
