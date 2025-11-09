Returns the number of leading ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((!(u24::MAX >> 2u32)).leading_ones(), 2);
assert_eq!(0_u24.leading_ones(), 0);
assert_eq!(u24::MAX.leading_ones(), 24);
# }
```
