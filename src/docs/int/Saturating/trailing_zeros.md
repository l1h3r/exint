Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating(0b00101000_i24).trailing_zeros(), 3);
# }
```
