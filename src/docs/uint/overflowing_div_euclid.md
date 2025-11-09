Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.

Returns a tuple of the divisor along with a boolean indicating whether an
arithmetic overflow would occur. Note that for unsigned integers overflow never
occurs, so the second value is always `false`. Since, for the positive integers,
all common definitions of division are equal, this is exactly equal to
`self.overflowing_div(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.overflowing_div_euclid(2_u24), (2_u24, false));
# }
```
