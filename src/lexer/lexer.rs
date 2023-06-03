use std::todo;

use crate::token::token::{self, Token};

struct Lexer{
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer{

    fn new(input: String) -> Lexer{
        let mut lex = Lexer{input: input.into_bytes(), position: 0, read_position: 0, ch: 0};
        lex.read_char();
        return lex;
}
   fn read_char(&mut self){
       if self.read_position >= self.input.len(){
           self.ch = 0;
       }
       else{
           self.ch = self.input[self.read_position]
       }
       self.position = self.read_position;
       self.read_position += 1;
   }

    fn next_token(&mut self) -> Token{
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => Token::ASSIGN,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            b'!' => Token::BANG,
            b'-' => Token::MINUS,
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'a'..=b'z' | b'A'..=b'Z' =>{
                let tok = self.read_identifier();
                return self.lookup_ident(tok);

            },
            b'0'..=b'9' => {
                self.read_number();
                return Token::INT

            },
            _ => Token::ILLEGAL,
        };
        self.read_char();
        return token;
    }

fn skip_whitespace(&mut self){
    while self.ch.is_ascii_whitespace(){
        self.read_char();
    }
}

fn read_identifier(&mut self) -> String{
    let position = self.position;
    while self.ch.is_ascii_alphabetic(){
        self.read_char();
    }
    return String::from_utf8(self.input[position..self.position].to_vec()).unwrap();
}

fn read_number(&mut self) -> String{
    let position = self.position;
    while self.ch.is_ascii_digit(){
        self.read_char();
    }
    return String::from_utf8(self.input[position..self.position].to_vec()).unwrap();
}

fn lookup_ident(&mut self, ident: String) -> Token{
    match ident.as_str(){
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENT,
    }
}



}
#[cfg(test)]
mod tests {
use std::print;

use super::*;
#[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
"#;
    let mut tests = vec![
    Token::LET,
    Token::IDENT,
    Token::ASSIGN,
    Token::INT,
    Token::SEMICOLON,
    Token::LET,
    Token::IDENT,
    Token::ASSIGN,
    Token::INT,
    Token::SEMICOLON,
    Token::LET,
    Token::IDENT,
    Token::ASSIGN,
    Token::FUNCTION,
    Token::LPAREN,
    Token::IDENT,
    Token::COMMA,
    Token::IDENT,
    Token::RPAREN,
    Token::LBRACE,
    Token::IDENT,
    Token::PLUS,
    Token::IDENT,
    Token::SEMICOLON,
    Token::RBRACE,
    Token::SEMICOLON,
    Token::LET,
    Token::IDENT,
    Token::ASSIGN,
    Token::IDENT,
    Token::LPAREN,
    Token::IDENT,
    Token::COMMA,
    Token::IDENT,
    Token::RPAREN,
    Token::SEMICOLON,
    Token::BANG,
    Token::MINUS,
    Token::SLASH,
    Token::ASTERISK,
    Token::INT,
    Token::SEMICOLON,
    Token::INT,
    Token::LT,
    Token::INT,
    Token::GT,
    Token::INT,
    Token::SEMICOLON,
    Token::EOF,
    ];
    let mut lex = Lexer::new(input.into());

    for t in tests{
        let tok = lex.next_token();
        println!("{:?} {:?}",tok, t);
        assert_eq!(tok, t);
    }

    }
}

