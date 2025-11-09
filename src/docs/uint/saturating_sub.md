Saturating integer subtraction. Computes `self - rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.saturating_sub(27_u24), 73_u24);
assert_eq!(13_u24.saturating_sub(127_u24), 0_u24);
# }
```
