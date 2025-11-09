Saturating integer multiplication. Computes `self * rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_u24.saturating_mul(10_u24), 20_u24);
assert_eq!((u24::MAX).saturating_mul(10_u24), u24::MAX);
# }
```
