#![feature(let_chains)]

/// ! This crate holds a proc_macro_attribute which can be used to create type aliases with a more inutive syntax (similar to the way struct aliases are handled) 
/// 
/// # Examples
/// ```
/// use generic_trait_alias::trait_alias;
///
/// pub trait Z {
///     fn z(&self) -> u8;
/// }
/// 
/// // Creates a pub trait alias
/// #[trait_alias]
/// pub type X = Z + Clone;
/// 
/// // Creates a private trait alias
/// #[trait_alias]
/// type A = Z + Clone;
/// ```

use lexer::Lexer;
use parser::Parser;
use proc_macro::TokenStream;

mod parser;
mod lexer;
mod generic;

/// Used to create a generic type alias containing multiple traits (similar to a nested struct type alias)
/// 
/// # Examples
/// ```
/// use generic_trait_alias::trait_alias;
///
/// 
/// // Define an internal trait
/// pub trait Z {
///     fn z(&self) -> u8;
/// }
/// 
/// // Creates a pub trait alias combining an internal trait with external one
/// #[trait_alias]
/// pub type X = Z + Clone;
/// 
/// // Creates a private trait alias
/// #[trait_alias]
/// type A = Z + Clone;
/// 
/// fn example_fn<T: A>(x: T) -> u8 {
///     return x.z() + x.z();
/// }
/// 
/// pub fn example_fn_pub<T: X>(x: T) -> u8 {
///     return x.z() + x.z();
/// }
/// ```
#[proc_macro_attribute]
pub fn trait_alias(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
        ", ident, trait_list, ident, trait_list);
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
        assert_eq!(15, lexer.tokens().len());
    }

    #[test]
    pub fn test_parser() {
        let binding = String::from("type Z = Add + std :: fmt :: Display + Y + L;");
        let buff: Vec<&str> = binding.split(" ").into_iter().collect();
        let mut lexer = Lexer::new(buff);

        lexer.get_token();
        let mut parser = Parser::new(lexer.tokens());
        let (_ident, traits) = parser.get_string_repr();
        assert_eq!("Add+std::fmt::Display+Y+L;", traits);
    }

    #[test]
    pub fn test_macro_logic() {

    }
}
