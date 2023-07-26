use std::{ops::Add, fmt::Display};

pub trait X {
    fn hello(&self) -> String;
}

pub trait Y {
    fn goodbye() -> String;
}
pub trait L {
    fn world() -> u8;
}

#[generic_trait_alias::generic_trait]
pub type Z = std::ops::Add + std::fmt::Display + Y + L;

#[test]
fn test_macro() {
    
}