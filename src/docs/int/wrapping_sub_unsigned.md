Wrapping (modular) subtraction with an unsigned integer. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_i24.wrapping_sub_unsigned(127_u24), -127_i24);
assert_eq!(-2_i24.wrapping_sub_unsigned(u24::MAX), -1_i24);
# }
```
