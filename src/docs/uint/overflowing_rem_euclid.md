Calculates the remainder `self.rem_euclid(rhs)` as if by Euclidean division.

Returns a tuple of the modulo after dividing along with a boolean indicating
whether an arithmetic overflow would occur. Note that for unsigned integers
overflow never occurs, so the second value is always `false`. Since, for the
positive integers, all common definitions of division are equal, this operation
is exactly equal to `self.overflowing_rem(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.overflowing_rem_euclid(2_u24), (1_u24, false));
# }
```
