Wrapping (modular) addition. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(200_u24.wrapping_add(55_u24), 255_u24);
assert_eq!(200_u24.wrapping_add(u24::MAX), 199_u24);
# }
```
