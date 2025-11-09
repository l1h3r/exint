Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0b00101000_u24.trailing_zeros(), 3);
assert_eq!(0_u24.trailing_zeros(), 24);
assert_eq!(u24::MAX.trailing_zeros(), 0);
# }
```
