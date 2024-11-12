pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &str = "INT"; // 1343456

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
pub type TokenType = String;
/// TODO Rustで定数なHashMapの定義の仕方
///
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}
use std::collections::HashMap;
pub fn lookup_ident(ident: &str) -> TokenType {
    dbg!(ident);
    let mut map = HashMap::new();
    map.insert("fn", FUNCTION);
    map.insert("let", LET);
    map.insert("true", TRUE);
    map.insert("false", FALSE);
    map.insert("if", IF);
    map.insert("else", ELSE);
    map.insert("return", RETURN);

    match map.get(ident) {
        Some(value) => value.to_string(),

        None => IDENT.to_string(),
    }
}
