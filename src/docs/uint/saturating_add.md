Saturating integer addition. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.saturating_add(1_u24), 101_u24);
assert_eq!(u24::MAX.saturating_add(127_u24), u24::MAX);
# }
```
