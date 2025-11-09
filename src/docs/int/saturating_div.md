Saturating integer division. Computes `self / rhs`,
saturating at the numeric bounds instead of overflowing.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.saturating_div(2_i24), 2_i24);
assert_eq!(i24::MAX.saturating_div(-1_i24), i24::MIN + 1_i24);
assert_eq!(i24::MIN.saturating_div(-1_i24), i24::MAX);
# }
```
