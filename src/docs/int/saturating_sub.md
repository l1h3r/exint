Saturating integer subtraction. Computes `self - rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.saturating_sub(127_i24), -27_i24);
assert_eq!(i24::MIN.saturating_sub(100_i24), i24::MIN);
assert_eq!(i24::MAX.saturating_sub(-1_i24), i24::MAX);
# }
```
