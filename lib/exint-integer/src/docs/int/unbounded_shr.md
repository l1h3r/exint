Unbounded shift right. Computes `self >> rhs`, without bounding the value of `rhs`.

If `rhs` is larger or equal to the number of bits in `self`,
the entire value is shifted out, which yields `0` for a positive number,
and `-1` for a negative number.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(0x10).unbounded_shr(4), int!(0x1));
assert_eq!(int!(0x10).unbounded_shr(129), int!(0));
assert_eq!(int::MIN.unbounded_shr(129), int!(-1));
```
