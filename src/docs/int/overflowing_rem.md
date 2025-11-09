Calculates the remainder when `self` is divided by `rhs`.

Returns a tuple of the remainder after dividing along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would occur then 0 is
returned.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.overflowing_rem(2_i24), (1_i24, false));
assert_eq!(i24::MIN.overflowing_rem(-1_i24), (0_i24, true));
# }
```
