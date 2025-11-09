Returns `self` with only the most significant bit set, or `0` if the input is `0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0b01100100_i24.isolate_highest_one(), 0b01000000_i24);
assert_eq!(0_i24.isolate_highest_one(), 0_i24);
# }
```
