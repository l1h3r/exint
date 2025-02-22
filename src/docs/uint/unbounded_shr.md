Unbounded shift right. Computes `self >> rhs`, without bounding the value of `rhs`.

If `rhs` is larger or equal to the number of bits in `self`,
the entire value is shifted out, and `0` is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(0x10).unbounded_shr(4), uint!(0x1));
assert_eq!(uint!(0x10).unbounded_shr(129), uint!(0));
```
