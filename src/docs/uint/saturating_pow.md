Saturating integer exponentiation. Computes `self.pow(exp)`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(4_u24.saturating_pow(3), 64_u24);
assert_eq!(u24::MAX.saturating_pow(2), u24::MAX);
# }
```
