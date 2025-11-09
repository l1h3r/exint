Unbounded shift left. Computes `self << rhs`, without bounding the value of `rhs`.

If `rhs` is larger or equal to the number of bits in `self`,
the entire value is shifted out, and `0` is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x1_i24.unbounded_shl(4), 0x10_i24);
assert_eq!(0x1_i24.unbounded_shl(129), 0_i24);
# }
```
