Computes `self >> rhs`, returning `None` if `rhs` >= `Self::BITS`
or if any non-zero bits would be shifted out.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x10_i24.shr_exact(4), Some(0x1_i24));
assert_eq!(0x10_i24.shr_exact(5), None);
# }
```
