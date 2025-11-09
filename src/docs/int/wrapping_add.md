Wrapping (modular) addition. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_add(27_i24), 127_i24);
assert_eq!(i24::MAX.wrapping_add(2_i24), i24::MIN + 1_i24);
# }
```
