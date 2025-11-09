Checked integer subtraction. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_u24.checked_sub(1_u24), Some(0_u24));
assert_eq!(0_u24.checked_sub(1_u24), None);
# }
```
