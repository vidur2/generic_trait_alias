use std::str::Chars;

use proc_macro::{TokenStream, Ident};

#[derive(Debug, Clone)]
pub enum Token {
    Type,
    Ident(String),
    Assign,
    SemiColon,
    Colon,
    Pub,
    Plus
}

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    idx: usize,
    jmp_idx: usize,
    buff: Vec<&'a str>
}

trait MatchToken<T> {
    fn match_token(&self, s: T) -> Option<Token>;
}

impl<'a> Lexer<'a> {
    pub fn new(buff: Vec<&'a str>) -> Self{
        return Self {
            tokens: Vec::new(),
            idx: 0,
            jmp_idx: 1,
            buff,
        }
    }

    pub fn get_token(&mut self) {
        for i in self.buff.iter() {
            self.tokens.push(self.match_token(i.to_string()).unwrap());
        }
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }
}

impl<'a> MatchToken<char> for Lexer<'a> {
    fn match_token(&self, s: char) -> Option<Token> {
        match s {
            ';' => return Some(Token::SemiColon),
            '=' => return Some(Token::Assign),
            '+' => return Some(Token::Plus),
            _ => return None
        }
    }
}

impl<'a> MatchToken<String> for Lexer<'a> {
    fn match_token(&self, s: String) -> Option<Token> {
        match s.as_str() {
            ";" => return Some(Token::SemiColon),
            "=" => return Some(Token::Assign),
            "+" => return Some(Token::Plus),
            "type" => return Some(Token::Type),
            "pub" => return Some(Token::Pub),
            "::" => return Some(Token::Colon),
            _ => return Some(Token::Ident(s.replace(" ", "")))
        };
    }
}