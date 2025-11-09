Returns the number of zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(i24::MAX.count_zeros(), 1);
# }
```
