Returns the number of leading ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((-1_i24).leading_ones(), i24::BITS);
# }
```
