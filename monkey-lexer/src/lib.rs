use monkey_token::{
    lookup_ident, Token, TokenType, ASSIGN, COMMA, ILLEGAL, INT, LBRACE, LPAREN, RBRACE, RPAREN,
    SEMICOLON,
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
        l.read_char()
    }
    fn read_char(&mut self) -> Lexer {
        if self.readPosition >= self.input.len() as u8 {
            self.ch = '\0';
        } else {
            if let Some(char_at_n) = self.input.chars().nth(self.readPosition.into()) {
                // charをバイト列に変換
                self.ch = char_at_n;
                // dbg!("{}のバイト列: {:?}", char_at_n, &self.ch);
                // dbg!("現在の状態: {:?}", &self);
            } else {
                // dbg!("文字列が短すぎます");
            }
        }
        self.position = self.readPosition;
        // dbg!(self.position, self.readPosition);
        self.readPosition += 1;
        // dbg!(self.position, self.readPosition);
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
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                token = Self::new_token(ASSIGN.to_owned(), self.ch.to_string());
                self.read_char();
                token
            }
            ';' => {
                token = Self::new_token(SEMICOLON.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            '(' => {
                token = Self::new_token(LPAREN.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            ')' => {
                token = Self::new_token(RPAREN.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            ',' => {
                token = Self::new_token(COMMA.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            '+' => {
                token = Self::new_token(monkey_token::PLUS.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            '{' => {
                token = Self::new_token(LBRACE.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            '}' => {
                token = Self::new_token(RBRACE.to_string(), self.ch.to_string());
                self.read_char();
                token
            }
            _ => {
                dbg!(self.ch);
                if self.is_letter() {
                    token.Literal = self.read_identifier();
                    token.Type = lookup_ident(&token.Literal);
                    // ここで一文字進んでいると下でもう1️⃣文字すすんでそ
                    token
                } else if self.is_digit() {
                    token.Type = INT.to_string();
                    token.Literal = self.read_number();
                    token
                } else {
                    Self::new_token(ILLEGAL.to_string(), self.ch.to_string())
                }
            }
        };

        dbg!(self.position);
        token
    }
    fn new_token(assign: TokenType, ch: String) -> Token {
        return Token {
            Type: assign,
            Literal: ch,
        };
    }

    ///
    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == '_'
    }
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char(); // 次の文字を読み込む
        }
        let identifier = &self.input[start_pos as usize..self.position as usize];
        dbg!(self.position);
        identifier.to_string() // 文字列を返す
    }

    /// スペースであるかぎりスキップする
    fn skip_whitespace(&mut self) -> () {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' {
            self.read_char();
        }
    }
    /// 0-9である限り
    fn is_digit(&self) -> bool {
        self.ch.is_numeric()
    }
    /// 数字だけを取得
    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.is_numeric() || self.ch == '_' {
            self.read_char(); // 次の文字を読み込む
        }
        let identifier = &self.input[start_pos as usize..self.position as usize];
        dbg!(self.position);
        identifier.to_string() // 文字列を返す
    }
}
