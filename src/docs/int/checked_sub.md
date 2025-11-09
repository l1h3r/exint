Checked integer subtraction. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((i24::MIN + 2_i24).checked_sub(1_i24), Some(i24::MIN + 1_i24));
assert_eq!((i24::MIN + 2_i24).checked_sub(3_i24), None);
# }
```
