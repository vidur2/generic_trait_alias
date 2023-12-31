# generic_trait_alias
Holds a proc_macro_attribute which can be used to create type aliases with a more inutive syntax (similar to the way struct aliases are handled) 

## Install
```bash
cargo install generic_trait_alias
```
or 
```bash
cargo add generic_trait_alias
```

## Examples
```rust
use generic_trait_alias::trait_alias;

// Define internal trait
pub trait Z {
    fn z(&self) -> u8;
}

// Creates a pub trait alias with internal and external traits
#[trait_alias]
pub type X = Z + Clone;

// Creates a private trait alias with internal and external traits
#[trait_alias]
type A = Z + Clone;

// Only works with public alias x
pub fn example_pub<T: X>(x: T) {
    println!("{}", x.z());
}

// Private functions can work with public or private alias
fn example<T: A>(x: A) {
    println!("{}", x.z());
}
```

## Limitations
Currently does not support combined generic traits