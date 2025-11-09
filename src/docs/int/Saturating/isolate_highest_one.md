Returns `self` with only the most significant bit set, or `0` if the input is `0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating(0b01100100_i24).isolate_highest_one(), Saturating(0b01000000_i24));
assert_eq!(Saturating(0_i24).isolate_highest_one(), Saturating(0_i24));
# }
```
