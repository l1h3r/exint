Returns `true` if `self` is negative and `false` if the number is zero or positive.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!((-10_i24).is_negative());
assert!(!10_i24.is_negative());
# }
```
