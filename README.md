# generic_trait_alias
Holds a proc_macro_attribute which can be used to create type aliases with a more inutive syntax (similar to the way struct aliases are handled) 

## Install
```bash
cargo install generic_trait_alias
```

## Examples
```rust
use generic_trait_alias::trait_alias;
 pub trait Z {
     fn z(&self) -> u8;
 }
 
 // Creates a pub trait alias
 #[trait_alias]
 pub type X = Z + Clone;
 
 // Creates a private trait alias
 #[trait_alias]
 type A = Z + Clone;

// Only works with public alias x
 pub fn example_pub<T: X>(x: T) {
    println!("{}", x.z());
 }
```