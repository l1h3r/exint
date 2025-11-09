Checked shift left. Computes `self << rhs`, returning `None`
if `rhs` is larger than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x1_u24.checked_shl(4), Some(0x10_u24));
assert_eq!(0x10_u24.checked_shl(129), None);
assert_eq!(0x10_u24.checked_shl(u24::BITS - 1), Some(0_u24));
# }
```
