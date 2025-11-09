Returns the number of trailing ones in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating(0b01010111_u24).trailing_ones(), 3);
# }
```
