Checked subtraction with an unsigned integer. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_i24.checked_sub_unsigned(2_u24), Some(-1_i24));
assert_eq!((i24::MIN + 2_i24).checked_sub_unsigned(3_u24), None);
# }
```
