Checked integer multiplication. Computes `self * rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.checked_mul(1_u24), Some(5_u24));
assert_eq!(u24::MAX.checked_mul(2_u24), None);
# }
```
