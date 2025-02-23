Checked integer subtraction. Computes `self - rhs` and checks if the result fits
into an [`int`], returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(10).checked_signed_diff(uint!(2)), Some(int!(8)));
assert_eq!(uint!(2).checked_signed_diff(uint!(10)), Some(int!(-8)));
assert_eq!(uint::MAX.checked_signed_diff(int::MAX.cast_unsigned()), None);
assert_eq!(int::MAX.cast_unsigned().checked_signed_diff(uint::MAX), Some(int::MIN));
assert_eq!((int::MAX.cast_unsigned() + uint!(1)).checked_signed_diff(uint!(0)), None);
assert_eq!(uint::MAX.checked_signed_diff(uint::MAX), Some(int!(0)));
```
