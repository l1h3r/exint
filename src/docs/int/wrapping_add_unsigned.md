Wrapping (modular) addition with an unsigned integer. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_add_unsigned(27_u24), 127_i24);
assert_eq!(i24::MAX.wrapping_add_unsigned(2_u24), i24::MIN + 1_i24);
# }
```
