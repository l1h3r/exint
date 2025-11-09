Saturating integer exponentiation. Computes `self.pow(exp)`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(-4_i24.saturating_pow(3), -64_i24);
assert_eq!(i24::MIN.saturating_pow(2), i24::MAX);
assert_eq!(i24::MIN.saturating_pow(3), i24::MIN);
# }
```
