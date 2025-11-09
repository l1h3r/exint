Checked integer division without remainder. Computes `self / rhs`,
returning `None` if `rhs == 0` or `self % rhs != 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(64_u24.checked_exact_div(2_u24), Some(32_u24));
assert_eq!(64_u24.checked_exact_div(32_u24), Some(2_u24));
assert_eq!(65_u24.checked_exact_div(2_u24), None);
# }
```
