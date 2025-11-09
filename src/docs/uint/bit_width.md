Returns the minimum number of bits required to represent `self`.

This method returns zero if `self` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.bit_width(), 0);
assert_eq!(0b111_u24.bit_width(), 3);
assert_eq!(0b1110_u24.bit_width(), 4);
assert_eq!(u24::MAX.bit_width(), 24);
# }
```
