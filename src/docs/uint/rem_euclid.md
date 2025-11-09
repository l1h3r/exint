Calculates the least remainder of `self (mod rhs)`.

Since, for the positive integers, all common definitions of division are equal,
this is exactly equal to `self % rhs`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(7_u24.rem_euclid(4_u24), 3_u24);
# }
```
