Checked integer remainder. Computes `self % rhs`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.checked_rem(2_i24), Some(1_i24));
assert_eq!(5_i24.checked_rem(0_i24), None);
assert_eq!(i24::MIN.checked_rem(-1_i24), None);
# }
```
