Shifts self left by `rhs` bits.

Returns a tuple of the shifted version of self along with a boolean indicating
whether the shift value was larger than or equal to the number of bits. If the
shift value is too large, then value is masked (N-1) where N is the number of
bits, and this value is then used to perform the shift.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(0x1).overflowing_shl(4), (uint!(0x10), false));
assert_eq!(uint!(0x1).overflowing_shl(132), (uint!(0x10), true));
assert_eq!(uint!(0x10).overflowing_shl(uint::BITS - 1), (uint!(0), false));
```
