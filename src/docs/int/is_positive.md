Returns `true` if `self` is positive and `false` if the number is zero or negative.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!(10_i24.is_positive());
assert!(!(-10_i24).is_positive());
# }
```
