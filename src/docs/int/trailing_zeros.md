Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((-4_i24).trailing_zeros(), 2);
# }
```
