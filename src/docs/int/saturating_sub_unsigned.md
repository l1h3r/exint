Saturating subtraction with an unsigned integer. Computes `self - rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.saturating_sub_unsigned(127_u24), -27_i24);
assert_eq!(i24::MIN.saturating_sub_unsigned(100_u24), i24::MIN);
# }
```
