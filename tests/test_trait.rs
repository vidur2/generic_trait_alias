pub trait X {
    fn hello(&self) -> String;
}

pub trait Y {
    fn goodbye(&self) -> String;
}
pub trait L {
    fn world(&self) -> u8;
}

#[generic_trait_alias::trait_alias]
pub type Z = Y + L;

pub struct Test();

impl Y for Test {
    fn goodbye(&self) -> String {
        return String::from("hello");
    }
}

impl L for Test {
    fn world(&self) -> u8 {
        return 7;
    }
}

pub fn test_fn<T: Z>(x: T) {
    println!("{} {}", x.goodbye(), x.world());
}

#[test]
fn test_macro() {
    test_fn(Test());
}