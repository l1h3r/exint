Wrapping (modular) multiplication. Computes `self * rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.wrapping_mul(12_i24), 120_i24);
assert_eq!(11_i8.wrapping_mul(12_i8), -124_i8);
# }
```
