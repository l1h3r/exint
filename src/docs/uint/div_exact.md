Integer division without remainder. Computes `self / rhs`,
returning `None` if `self % rhs != 0`.

# Panics

This function will panic  if `rhs == 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(64_u24.div_exact(2_u24), Some(32_u24));
assert_eq!(64_u24.div_exact(32_u24), Some(2_u24));
assert_eq!(65_u24.div_exact(2_u24), None);
# }
```
