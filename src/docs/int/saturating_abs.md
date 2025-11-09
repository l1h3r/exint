Saturating absolute value. Computes `self.abs()`,
returning `MAX` if `self == MIN` instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.saturating_abs(), 100_i24);
assert_eq!((-100_i24).saturating_abs(), 100_i24);
assert_eq!(i24::MIN.saturating_abs(), i24::MAX);
assert_eq!((i24::MIN + 1_i24).saturating_abs(), i24::MAX);
# }
```
