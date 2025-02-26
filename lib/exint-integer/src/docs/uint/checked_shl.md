Checked shift left. Computes `self << rhs`, returning `None`
if `rhs` is larger than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(0x1).checked_shl(4), Some(uint!(0x10)));
assert_eq!(uint!(0x10).checked_shl(129), None);
assert_eq!(uint!(0x10).checked_shl(uint::BITS - 1), Some(uint!(0)));
```
