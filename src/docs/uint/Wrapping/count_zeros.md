Returns the number of zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping(!0_u24).count_zeros(), 0);
# }
```
