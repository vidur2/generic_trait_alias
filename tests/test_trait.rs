pub trait X {
    fn bar(&self) -> String;
}

pub trait Y {
    fn foo(&self) -> String;
}
pub trait L {
    fn baz(&self) -> u8;
}

#[generic_trait_alias::trait_alias]
pub type Z = Y + L;

pub struct Test();

impl Y for Test {
    fn foo(&self) -> String {
        return String::from("hello");
    }
}

impl L for Test {
    fn baz(&self) -> u8 {
        return 7;
    }
}

pub fn test_fn<T: Z>(x: T) {
    println!("{} {}", x.foo(), x.baz());
}

#[test]
fn test_macro() {
    test_fn(Test());
}