Wrapping (modular) remainder. Computes `self % rhs`.

Wrapped remainder calculation on unsigned types is just the regular remainder
calculation. There's no way wrapping could ever happen. This function exists, so
that all operations are accounted for in the wrapping operations.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.wrapping_rem(10_u24), 0_u24);
# }
```
