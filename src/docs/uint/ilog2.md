Returns the base 2 logarithm of the number, rounded down.

# Panics

This function will panic if `self` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_u24.ilog2(), 1);
# }
```
