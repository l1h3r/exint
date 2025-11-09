Wrapping (modular) multiplication. Computes `self * rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_u24.wrapping_mul(12_u24), 120_u24);
assert_eq!(25_u8.wrapping_mul(12_u8), 44_u8);
# }
```
