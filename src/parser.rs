/*
Target Syntax:
type X = X1 + X2 + X3;
*/

use crate::{lexer::Token, generic::{self, GenericTrait}};

pub struct Parser {
    items: Vec<Token>,
}

impl Parser {
    pub fn new(items: Vec<Token>) -> Self { Self { items } }

    pub fn get_string_repr(&mut self) -> (GenericTrait, String) {
        let is_pub = self.get_is_pub();
        let mut generic = self.parse();
        generic.set_is_pub(is_pub);
        let mut trait_ls = String::new();
        for trait_repr in generic.traits() {
            trait_ls += &format!("{} + ", trait_repr);
        }

        trait_ls = trait_ls[0..trait_ls.len() - 2].to_string();

        return (generic, trait_ls);
    }

    fn get_is_pub(&mut self) -> bool {
        if let Token::Pub = self.items[0] {
            self.items = self.items[1..].to_vec();
            return true;
        }

        return false;
    }

    fn parse(&mut self) -> GenericTrait {
        let mut generic_trait = GenericTrait::new();

        if self.items.len() < 4 {
            panic!("Invalid expression, must have at least four tokens")
        }

        match self.items[0] {
            Token::Type => println!("First token valid"),
            _ => {
                panic!("Expected keyword type at position 0");
            }
        }

        if let Token::Ident(ident) = &self.items[1] {
            *generic_trait.ident_mut() = ident.clone();
        } else {
            panic!("Must have an identifier token at position 1")
        }

        match self.items[2] {
            Token::Assign => println!("Third token valid"),
            _ => panic!("Expected '=' at position 2")
        }

        for i in 3..self.items.len() {
            if let Token::Ident(ident) = &self.items[i] {
                generic_trait.traits_mut().push(ident.clone());
            } else if let Token::SemiColon = &self.items[i] {
                return generic_trait;
            } else if let Token::Assign = &self.items[i] {
                panic!("Expected trait identifier at position {}", i);
            }
        }

        return generic_trait;
    }
}