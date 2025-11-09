Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping(0b00101000_i24).trailing_zeros(), 3);
# }
```
