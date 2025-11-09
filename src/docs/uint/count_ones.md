Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0b01001100_u24.count_ones(), 3);
assert_eq!(0_u24.count_ones(), 0);
assert_eq!(u24::MAX.count_ones(), 24);
# }
```
