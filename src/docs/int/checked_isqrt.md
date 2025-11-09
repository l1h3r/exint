Returns the square root of the number, rounded down.

Returns `None` if `self` is negative.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.checked_isqrt(), Some(3_i24));
# }
```
