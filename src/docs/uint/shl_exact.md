Computes `self << rhs`, returning `None` if `rhs` >= `Self::BITS`
or if any non-zero bits would be shifted out.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x1_u24.shl_exact(4), Some(0x10_u24));
assert_eq!(0x1_u24.shl_exact(129), None);
# }
```
