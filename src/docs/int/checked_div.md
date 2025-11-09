Checked integer division. Computes `self / rhs`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((i24::MIN + 1_i24).checked_div(-1_i24), Some(i24::MAX));
assert_eq!(i24::MIN.checked_div(-1_i24), None);
assert_eq!(1_i24.checked_div(0_i24), None);
# }
```
