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
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let a = int!(7);
let b = int!(4);

assert_eq!(a.div_euclid(b), int!(1)); // 7 >= 4 * 1
assert_eq!(a.div_euclid(-b), int!(-1)); // 7 >= -4 * -1
assert_eq!((-a).div_euclid(b), int!(-2)); // -7 >= 4 * -2
assert_eq!((-a).div_euclid(-b), int!(2)); // -7 >= -4 * 2
```
