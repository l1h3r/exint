Returns `self` with only the most significant bit set, or `0` if the input is `0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping(0b01100100_u24).isolate_highest_one(), Wrapping(0b01000000_u24));
assert_eq!(Wrapping(0_u24).isolate_highest_one(), Wrapping(0_u24));
# }
```
