use std::ops::Add;

pub trait X {
    fn hello(&self) -> String;
}

pub trait Y {
    fn goodbye() -> String;
}
pub trait L {
    fn world() -> u8;
}

struct Test();

#[generic_trait_alias::generic_trait]
type Z = X + Y + L;

#[test]
fn test_macro() {
    
}