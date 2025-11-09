Wrapping (modular) subtraction with a signed integer. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_u24.wrapping_sub_signed(2_i24), u24::MAX);
assert_eq!(1_u24.wrapping_sub_signed(-2_i24), 3_u24);
assert_eq!((u24::MAX - 2_u24).wrapping_sub_signed(-4_i24), 1_u24);
# }
```
