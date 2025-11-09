Returns the number of zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.count_zeros(), 24);
assert_eq!(u24::MAX.count_zeros(), 0);
# }
```
