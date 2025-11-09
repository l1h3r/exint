Returns the number of trailing ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0b01010111_u24.trailing_ones(), 3);
assert_eq!(0_u24.trailing_ones(), 0);
assert_eq!(u24::MAX.trailing_ones(), 24);
# }
```
