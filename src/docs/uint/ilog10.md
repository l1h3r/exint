Returns the base 10 logarithm of the number, rounded down.

# Panics

This function will panic if `self` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_u24.ilog10(), 1);
# }
```
