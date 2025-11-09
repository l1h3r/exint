Calculates the quotient of `self` and `rhs`, rounding the result towards positive infinity.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(7_u24.div_ceil(4_u24), 2_u24);
# }
```
