Converts a [`uint<1>`] into a [`char`].

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let u = uint!(32 u8);
let c = char::from(u);
assert!(4 == std::mem::size_of_val(&c))
```
