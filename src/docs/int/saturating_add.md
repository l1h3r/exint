Saturating integer addition. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.saturating_add(1_i24), 101_i24);
assert_eq!(i24::MAX.saturating_add(100_i24), i24::MAX);
assert_eq!(i24::MIN.saturating_add(-1_i24), i24::MIN);
# }
```
