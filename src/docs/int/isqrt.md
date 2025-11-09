Returns the square root of the number, rounded down.

# Panics

This function will panic if `self` is negative.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.isqrt(), 3_i24);
# }
```
