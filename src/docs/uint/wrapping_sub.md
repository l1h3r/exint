Wrapping (modular) subtraction. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(100).wrapping_sub(uint!(100)), uint!(0));
assert_eq!(uint!(100).wrapping_sub(uint::MAX), uint!(101));
```
