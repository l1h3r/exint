Wrapping (modular) division. Computes `self / rhs`,
wrapping around at the boundary of the type.

The only case where such wrapping can occur is when one divides `MIN / -1` on a
signed type (where `MIN` is the negative minimal value for the type); this is
equivalent to `-MIN`, a positive value that is too large to represent in the
type. In such a case, this function returns `MIN` itself.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_div(10_i24), 10_i24);
assert_eq!((-128_i8).wrapping_div(-1_i8), -128_i8);
# }
```
