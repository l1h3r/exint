Checked subtraction with a signed integer. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_u24.checked_sub_signed(2_i24), None);
assert_eq!(1_u24.checked_sub_signed(-2_i24), Some(3_u24));
assert_eq!((u24::MAX - 2_u24).checked_sub_signed(-4_i24), None);
# }
```
