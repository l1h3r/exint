Checked integer subtraction. Computes `self - rhs` and checks if the result fits
into an [`int`], returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_u24.checked_signed_diff(2_u24), Some(8_i24));
assert_eq!(2_u24.checked_signed_diff(10_u24), Some(-8_i24));
assert_eq!(u24::MAX.checked_signed_diff(i24::MAX.cast_unsigned()), None);
assert_eq!(i24::MAX.cast_unsigned().checked_signed_diff(u24::MAX), Some(i24::MIN));
assert_eq!((i24::MAX.cast_unsigned() + 1_u24).checked_signed_diff(0_u24), None);
assert_eq!(u24::MAX.checked_signed_diff(u24::MAX), Some(0_i24));
# }
```
