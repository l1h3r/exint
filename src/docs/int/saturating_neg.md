Saturating integer negation. Computes `-self`,
returning `MAX` if `self == MIN` instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.saturating_neg(), -100_i24);
assert_eq!(-100_i24.saturating_neg(), 100_i24);
assert_eq!(i24::MIN.saturating_neg(), i24::MAX);
assert_eq!(i24::MAX.saturating_neg(), i24::MIN + 1_i24);
# }
```
