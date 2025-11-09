Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(3_u24.wrapping_pow(5), 243_u24);
assert_eq!(3_u8.wrapping_pow(6), 217_u8);
# }
```
