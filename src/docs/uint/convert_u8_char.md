Converts a [`uint<1>`] into a [`char`].

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let u = 32_u8;
let c = char::from(u);
assert!(4 == size_of_val(&c))
# }
```
