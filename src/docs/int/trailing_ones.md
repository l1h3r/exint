Returns the number of trailing ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(3_i24.trailing_ones(), 2);
# }
```
