Wrapping (modular) subtraction. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(0).wrapping_sub(int!(127)), int!(-127));
assert_eq!(int!(-2).wrapping_sub(int::MAX), int::MAX);
```
