Calculates the divisor when `self` is divided by `rhs`.

Returns a tuple of the divisor along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would occur then self is
returned.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.overflowing_div(2_i24), (2_i24, false));
assert_eq!(i24::MIN.overflowing_div(-1_i24), (i24::MIN, true));
# }
```
