Returns the logarithm of the number with respect to an arbitrary base, rounded down.

This method might not be optimized owing to implementation details; `ilog2` can
produce results more efficiently for base 2, and `ilog10` can produce results
more efficiently for base 10.

# Panics

This function will panic if `self` is less than or equal to zero, or if `base`
is less than 2.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.ilog(5_i24), 1);
# }
```
