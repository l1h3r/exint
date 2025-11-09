Calculates the quotient of `self` and `rhs`, rounding the result towards negative infinity.

This is the same as performing `self / rhs` for all unsigned integers.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(7_u24.div_floor(4_u24), 1_u24);
# }
```
