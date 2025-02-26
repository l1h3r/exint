Wrapping Euclidean division. Computes `self.div_euclid(rhs)`.

Wrapped division on unsigned types is just normal division. There's no way
wrapping could ever happen. This function exists, so that all operations are
accounted for in the wrapping operations. Since, for the positive integers, all
common definitions of division are equal, this is exactly equal to
`self.wrapping_div(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(100).wrapping_div_euclid(uint!(10)), uint!(10));
```
