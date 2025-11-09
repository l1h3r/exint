Returns the base 10 logarithm of the number, rounded down.

# Panics

This function will panic if `self` is less than or equal to zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.ilog10(), 1);
# }
```
