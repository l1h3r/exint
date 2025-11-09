Wrapping (modular) subtraction. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.wrapping_sub(100_u24), 0_u24);
assert_eq!(100_u24.wrapping_sub(u24::MAX), 101_u24);
# }
```
