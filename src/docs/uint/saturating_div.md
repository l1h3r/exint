Saturating integer division. Computes `self / rhs`,
saturating at the numeric bounds instead of overflowing.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.saturating_div(2_u24), 2_u24);
# }
```
