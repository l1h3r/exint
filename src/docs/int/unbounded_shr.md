Unbounded shift right. Computes `self >> rhs`, without bounding the value of `rhs`.

If `rhs` is larger or equal to the number of bits in `self`,
the entire value is shifted out, which yields `0` for a positive number,
and `-1` for a negative number.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x10_i24.unbounded_shr(4), 0x1_i24);
assert_eq!(0x10_i24.unbounded_shr(129), 0_i24);
assert_eq!(i24::MIN.unbounded_shr(129), -1_i24);
# }
```
