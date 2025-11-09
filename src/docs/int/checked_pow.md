Checked exponentiation. Computes `self.pow(exp)`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(8_i24.checked_pow(2), Some(64_i24));
assert_eq!(i24::MAX.checked_pow(2), None);
# }
```
