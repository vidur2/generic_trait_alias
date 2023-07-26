use std::str::Chars;

use proc_macro::{TokenStream, Ident};

#[derive(Debug)]
pub enum Token {
    Type,
    Ident(String),
    Assign,
    SemiColon,
    Space,
    Plus
}

pub struct Lexer {
    tokens: Vec<Token>,
    idx: usize,
    jmp_idx: usize,
    buff: Vec<char>
}

trait MatchToken<T> {
    fn match_token(&self, s: T) -> Option<Token>;
}

impl Lexer {
    pub fn new(buff: String) -> Self{
        return Self {
            tokens: Vec::new(),
            idx: 0,
            jmp_idx: 1,
            buff: buff.chars().into_iter().collect(),
        }
    }

    pub fn get_token(&mut self) {
        while self.get_next() {
            self.idx = self.jmp_idx + 1;
            self.jmp_idx = self.idx + 1;
        }
        // let s: String = self.buff[self.idx..self.jmp_idx].into_iter().collect();
        // self.tokens.push(self.match_token(s).unwrap());
    }

    fn peek(&self) -> (Option<Token>, bool) {
        if self.jmp_idx == self.buff.len() {
            return (None, false)
        }
        return (self.match_token(self.buff[self.jmp_idx]), true);
    }

    fn get_next(&mut self) -> bool {

        if self.idx < self.buff.len() && let Some(tok) = self.match_token(self.buff[self.idx]) {
            self.tokens.push(tok);
            self.idx += 1;
            self.jmp_idx += 1;
        }

        while self.jmp_idx < self.buff.len() && self.buff[self.jmp_idx] != ' ' {
            if self.jmp_idx >= self.buff.len() {
                return false;
            }

            if let (Some(tok), out) = self.peek() {
                let s: String = self.buff[self.idx..self.jmp_idx].into_iter().collect();
                self.tokens.push(self.match_token(s).unwrap());
                self.tokens.push(tok);
                return out;
            }
            self.jmp_idx += 1;
        }

        if self.jmp_idx <= self.buff.len() {
            let s: String = self.buff[self.idx..self.jmp_idx].into_iter().collect();
            self.tokens.push(self.match_token(s).unwrap());
            return self.jmp_idx < self.buff.len()
        }
        
        return false;
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }
}

impl MatchToken<char> for Lexer {
    fn match_token(&self, s: char) -> Option<Token> {
        match s {
            ';' => return Some(Token::SemiColon),
            '=' => return Some(Token::Assign),
            '+' => return Some(Token::Plus),
            _ => return None
        }
    }
}

impl MatchToken<String> for Lexer {
    fn match_token(&self, s: String) -> Option<Token> {
        match s.as_str() {
            ";" => return Some(Token::SemiColon),
            "=" => return Some(Token::Assign),
            "+" => return Some(Token::Plus),
            "type" => return Some(Token::Type),
            _ => return Some(Token::Ident(s.replace(" ", "")))
        };
    }
}