Computes `self << rhs`, returning `None` if `rhs` >= `Self::BITS`
or if any bits that would be shifted out differ from the resulting sign bit.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x1_i24.shl_exact(4), Some(0x10_i24));
assert_eq!(0x1_i24.shl_exact(i24::BITS - 2), Some(1_i24 << i24::BITS - 2));
assert_eq!(0x1_i24.shl_exact(i24::BITS - 1), None);
assert_eq!(-0x2_i24.shl_exact(i24::BITS - 2), Some(-0x2_i24 << i24::BITS - 2));
assert_eq!(-0x2_i24.shl_exact(i24::BITS - 1), None);
# }
```
