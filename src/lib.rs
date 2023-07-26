#![feature(let_chains)]

use lexer::Lexer;
use parser::Parser;
use proc_macro::TokenStream;

mod parser;
mod lexer;
mod generic;

#[proc_macro_attribute]
pub fn generic_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut lexer = Lexer::new(item.to_string().parse().unwrap());

    lexer.get_token();

    let mut parser = Parser::new(lexer.tokens());

    let (generic, trait_list) = parser.get_string_repr();
    let ident = generic.ident();

    let expanded = format!("
        trait {}: {} {{}}
        impl<T> {} for T where T: {} {{}}
    ", ident, trait_list, ident, trait_list);

    return expanded.parse().unwrap()
}

#[cfg(test)]
mod test {

    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_lexer() {
        let mut lexer = Lexer::new("type Z<T> = Add<T> + X + Y + L;".to_string().parse().unwrap());
        lexer.get_token();
        println!("{:?}", lexer.tokens());
    }

    #[test]
    pub fn test_parser() {
        let mut lexer = Lexer::new("type Z<T> = Add<T> + X + Y + L;".to_string().parse().unwrap());

        lexer.get_token();
        let mut parser = Parser::new(lexer.tokens());
        let (ident, traits) = parser.get_string_repr();
        println!("{}", traits);
    }

    #[test]
    pub fn test_macro_logic() {

    }
}
