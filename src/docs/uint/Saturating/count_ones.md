Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating(0b01001100_u24).count_ones(), 3);
# }
```
