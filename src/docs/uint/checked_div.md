Checked integer division. Computes `self / rhs`,
returning `None` if `rhs == 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(128_u24.checked_div(2_u24), Some(64_u24));
assert_eq!(1_u24.checked_div(0_u24), None);
# }
```
