Checked addition with an unsigned integer. Computes `self + rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_i24.checked_add_unsigned(2_u24), Some(3_i24));
assert_eq!((i24::MAX - 2_i24).checked_add_unsigned(3_u24), None);
# }
```
