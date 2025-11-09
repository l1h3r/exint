Checked shift right. Computes `self >> rhs`, returning `None`
if `rhs` is larger than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x10_i24.checked_shr(4), Some(0x1_i24));
assert_eq!(0x10_i24.checked_shr(128), None);
# }
```
