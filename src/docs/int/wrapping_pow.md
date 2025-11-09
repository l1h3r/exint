Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(3_i24.wrapping_pow(4), 81_i24);
assert_eq!(3_i8.wrapping_pow(5), -13_i8);
assert_eq!(3_i8.wrapping_pow(6), -39_i8);
# }
```
