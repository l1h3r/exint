Returns `self` with only the least significant bit set, or `0` if the input is `0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0b01100100_i24.isolate_lowest_one(), 0b00000100_i24);
assert_eq!(0_i24.isolate_lowest_one(), 0_i24);
# }
```
