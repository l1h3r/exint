Returns the base 10 logarithm of the number, rounded down.

Returns `None` if the number is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_u24.checked_ilog10(), Some(1));
# }
```
