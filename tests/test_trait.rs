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
pub type Z = Y + L + std::ops::Add<u8>;

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

impl std::ops::Add<u8> for Test {
    type Output = u8;

    fn add(self, rhs: u8) -> Self::Output {
        return self.baz() + rhs
    }
}

pub fn test_fn<T: Z>(x: T) -> String {
    return format!("{} {}", x.foo(), x.baz());
}

#[test]
fn test_macro() {
    assert_eq!(test_fn(Test()), String::from("hello 7"))
}