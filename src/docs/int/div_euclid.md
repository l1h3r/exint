Calculates the quotient of Euclidean division of `self` by `rhs`.

This computes the integer `q` such that `self = q * rhs + r`, with
`r = self.rem_euclid(rhs)` and `0 <= r < abs(rhs)`.

In other words, the result is `self / rhs` rounded to the integer `q` such that
`self >= q * rhs`.
If `self > 0`, this is equal to rounding towards zero (the default in Rust);
if `self < 0`, this is equal to rounding away from zero (towards +/- infinity).
If `rhs > 0`, this is equal to rounding towards -infinity;
if `rhs < 0`, this is equal to rounding towards +infinity.

# Panics

This function will panic if `rhs` is zero or if `self` is `Self::MIN` and `rhs`
is -1. This behavior is not affected by the `overflow-checks` flag.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let a = 7_i24;
let b = 4_i24;

assert_eq!(a.div_euclid(b), 1_i24); // 7 >= 4 * 1
assert_eq!(a.div_euclid(-b), -1_i24); // 7 >= -4 * -1
assert_eq!((-a).div_euclid(b), -2_i24); // -7 >= 4 * -2
assert_eq!((-a).div_euclid(-b), 2_i24); // -7 >= -4 * 2
# }
```
