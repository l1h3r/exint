Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0b01000000_i24.count_ones(), 1);
# }
```
