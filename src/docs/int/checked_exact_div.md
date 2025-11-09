Checked integer division without remainder. Computes `self / rhs`,
returning `None` if `rhs == 0`, the division results in overflow,
or `self % rhs != 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((i24::MIN + 1_i24).checked_exact_div(-1_i24), Some(i24::MAX));
assert_eq!(-5_i24.checked_exact_div(2_i24), None);
assert_eq!(i24::MIN.checked_exact_div(-1_i24), None);
assert_eq!(1_i24.checked_exact_div(0_i24), None);
# }
```
