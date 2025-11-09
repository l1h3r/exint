Returns the logarithm of the number with respect to an arbitrary base, rounded down.

Returns `None` if the number is negative or zero, or if the base is not at least 2.

This method might not be optimized owing to implementation details;
`checked_ilog2` can produce results more efficiently for base 2, and
`checked_ilog10` can produce results more efficiently for base 10.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.checked_ilog(5_i24), Some(1));
# }
```
