Checked integer multiplication. Computes `self * rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(i24::MAX.checked_mul(1_i24), Some(i24::MAX));
assert_eq!(i24::MAX.checked_mul(2_i24), None);
# }
```
