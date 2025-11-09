Checked integer remainder. Computes `self % rhs`,
returning `None` if `rhs == 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.checked_rem(2_u24), Some(1_u24));
assert_eq!(5_u24.checked_rem(0_u24), None);
# }
```
