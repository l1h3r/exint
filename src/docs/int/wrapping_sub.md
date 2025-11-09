Wrapping (modular) subtraction. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_i24.wrapping_sub(127_i24), -127_i24);
assert_eq!(-2_i24.wrapping_sub(i24::MAX), i24::MAX);
# }
```
