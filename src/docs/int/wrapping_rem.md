Wrapping (modular) remainder. Computes `self % rhs`,
wrapping around at the boundary of the type.

Such wrap-around never actually occurs mathematically; implementation artifacts
make `x % y` invalid for `MIN / -1` on a signed type (where `MIN` is the
negative minimal value). In such a case, this function returns `0`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_rem(10_i24), 0_i24);
assert_eq!((-128_i8).wrapping_rem(-1_i8), 0_i8);
# }
```
