Unbounded shift right. Computes `self >> rhs`, without bounding the value of `rhs`.

If `rhs` is larger or equal to the number of bits in `self`,
the entire value is shifted out, and `0` is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x10_u24.unbounded_shr(4), 0x1_u24);
assert_eq!(0x10_u24.unbounded_shr(129), 0_u24);
# }
```
