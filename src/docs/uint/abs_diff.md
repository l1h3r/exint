Computes the absolute difference between `self` and `rhs`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(100).abs_diff(uint!(80)), uint!(20));
assert_eq!(uint!(100).abs_diff(uint!(110)), uint!(10));
```
