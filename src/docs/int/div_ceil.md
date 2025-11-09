Calculates the quotient of `self` and `rhs`, rounding the result towards
positive infinity.

# Panics

This function will panic if `rhs` is zero or if `self` is `Self::MIN` and
`rhs` is -1. This behavior is not affected by the `overflow-checks` flag.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let a = 8_i24;
let b = 3_i24;

assert_eq!(a.div_ceil(b), 3_i24);
assert_eq!(a.div_ceil(-b), -2_i24);
assert_eq!((-a).div_ceil(b), -2_i24);
assert_eq!((-a).div_ceil(-b), 3_i24);
# }
```
