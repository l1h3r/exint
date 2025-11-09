Returns the base 2 logarithm of the number, rounded down.

Returns `None` if the number is negative or zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_i24.checked_ilog2(), Some(1));
# }
```
