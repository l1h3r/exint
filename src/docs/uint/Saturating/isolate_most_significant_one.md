Returns `self` with only the most significant bit set, or `0` if the input is `0`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(Saturating(uint!(0b01100100)).isolate_most_significant_one(), Saturating(uint!(0b01000000)));
assert_eq!(Saturating(uint!(0)).isolate_most_significant_one(), Saturating(uint!(0)));
```
