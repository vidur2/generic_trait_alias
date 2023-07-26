#![feature(let_chains)]

use lexer::Lexer;
use parser::Parser;
use proc_macro::TokenStream;

mod parser;
mod lexer;
mod generic;

#[proc_macro_attribute]
pub fn generic_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let binding = item.to_string();
    let buff: Vec<&str> = binding.split(" ").into_iter().collect();
    let mut lexer = Lexer::new(buff);

    lexer.get_token();

    let mut parser = Parser::new(lexer.tokens());

    let (generic, trait_list) = parser.get_string_repr();
    let ident = generic.ident();
    let mut expanded = format!("
        trait {}: {} {{}}
        impl<T> {} for T where T: {} {{}}
    ", ident, trait_list, ident, trait_list);

    if generic.is_pub() {
        expanded = format!("
            pub trait {}: {} {{}}
            impl<T> {} for T where T: {} {{}}
        ", ident, "Clone + std::fmt::Display + Y + L", ident, "Clone + std::fmt::Display + Y + L");
    }

    return expanded.parse().unwrap()
}

#[cfg(test)]
mod test {

    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_lexer() {
        let binding = String::from("type Z = Add + std :: fmt :: Display + Y + L;");
        let buff: Vec<&str> = binding.split(" ").into_iter().collect();
        let mut lexer = Lexer::new(buff);
        lexer.get_token();
        println!("{:?}", lexer.tokens());
    }

    #[test]
    pub fn test_parser() {
        let binding = String::from("type Z = Add + std :: fmt :: Display + Y + L;");
        let buff: Vec<&str> = binding.split(" ").into_iter().collect();
        let mut lexer = Lexer::new(buff);

        lexer.get_token();
        let mut parser = Parser::new(lexer.tokens());
        let (ident, traits) = parser.get_string_repr();
        println!("{}", traits);
    }

    #[test]
    pub fn test_macro_logic() {

    }
}
