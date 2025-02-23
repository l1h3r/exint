Calculates the quotient of `self` and `rhs`, rounding the result towards positive infinity.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(7).div_ceil(uint!(4)), uint!(2));
```
