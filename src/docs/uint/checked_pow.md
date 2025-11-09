Checked exponentiation. Computes `self.pow(exp)`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_u24.checked_pow(5), Some(32_u24));
assert_eq!(u24::MAX.checked_pow(2), None);
# }
```
