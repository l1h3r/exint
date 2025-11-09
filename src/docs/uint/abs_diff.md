Computes the absolute difference between `self` and `rhs`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.abs_diff(80_u24), 20_u24);
assert_eq!(100_u24.abs_diff(110_u24), 10_u24);
# }
```
