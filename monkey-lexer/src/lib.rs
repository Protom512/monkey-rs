use std::str::Bytes;

use monkey_token::{
    Token, TokenType, ASSIGN, COMMA, EOF, LBRACE, LPAREN, RBRACE, RPAREN, SEMICOLON,
};
#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: u8,
    readPosition: u8,
    ch: char,
}
impl Lexer {
    ///
    ///
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            readPosition: 0,
            ch: '0',
        };
        l.readChar()
    }
    fn readChar(&mut self) -> Lexer {
        if self.readPosition >= self.input.len() as u8 {
            self.ch = '\0';
        } else {
            if let Some(char_at_n) = self.input.chars().nth(self.readPosition.into()) {
                // charをバイト列に変換
                self.ch = char_at_n;
                dbg!("{}のバイト列: {:?}", char_at_n, &self.ch);
                dbg!("現在の状態: {:?}", &self);
            } else {
                dbg!("文字列が短すぎます");
            }
        }
        self.position = self.readPosition;
        dbg!(self.position, self.readPosition);
        self.readPosition += 1;
        dbg!(self.position, self.readPosition);
        Lexer {
            position: self.position,
            readPosition: self.readPosition,
            ch: self.ch,
            input: self.input.clone(),
        }
    }
    pub fn NextToken(&mut self) -> Token {
        let mut token = Token {
            Type: "".to_string(),
            Literal: "".to_owned(),
        };
        let token = match self.ch {
            '=' => Self::newToken(ASSIGN.to_owned(), self.ch.to_string()),
            ';' => Self::newToken(SEMICOLON.to_string(), self.ch.to_string()),
            '(' => Self::newToken(LPAREN.to_string(), self.ch.to_string()),
            ')' => Self::newToken(RPAREN.to_string(), self.ch.to_string()),
            ',' => Self::newToken(COMMA.to_string(), self.ch.to_string()),
            '+' => Self::newToken(monkey_token::PLUS.to_string(), self.ch.to_string()),
            '{' => Self::newToken(LBRACE.to_string(), self.ch.to_string()),
            '}' => Self::newToken(RBRACE.to_string(), self.ch.to_string()),
            _ => {
                token.Literal = String::from("");
                token.Type = EOF.to_string();
                token
            }
        };
        self.readChar();
        token
    }
    fn newToken(assign: TokenType, ch: String) -> Token {
        return Token {
            Type: assign,
            Literal: ch,
        };
    }
}
