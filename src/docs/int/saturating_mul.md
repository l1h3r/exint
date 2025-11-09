Saturating integer multiplication. Computes `self * rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.saturating_mul(12_i24), 120_i24);
assert_eq!(i24::MAX.saturating_mul(10_i24), i24::MAX);
assert_eq!(i24::MIN.saturating_mul(10_i24), i24::MIN);
# }
```
