Checked integer addition. Computes `self + rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((u24::MAX - 2_u24).checked_add(1_u24), Some(u24::MAX - 1_u24));
assert_eq!((u24::MAX - 2_u24).checked_add(3_u24), None);
# }
```
