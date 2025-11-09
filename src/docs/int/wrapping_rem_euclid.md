Wrapping Euclidean remainder. Computes `self.rem_euclid(rhs)`,
wrapping around at the boundary of the type.

Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the
negative minimal value for the type). In this case, this method returns 0.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_rem_euclid(10_i24), 0_i24);
assert_eq!((-128_i8).wrapping_rem_euclid(-1_i8), 0_i8);
# }
```
