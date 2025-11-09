Checked integer addition. Computes `self + rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((i24::MAX - 2_i24).checked_add(1_i24), Some(i24::MAX - 1_i24));
assert_eq!((i24::MAX - 2_i24).checked_add(3_i24), None);
# }
```
