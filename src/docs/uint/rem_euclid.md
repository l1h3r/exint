Calculates the least remainder of `self (mod rhs)`.

Since, for the positive integers, all common definitions of division are equal,
this is exactly equal to `self % rhs`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(7).rem_euclid(uint!(4)), uint!(3));
```
