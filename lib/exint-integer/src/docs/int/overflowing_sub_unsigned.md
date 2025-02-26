Calculates `self` - `rhs` with an unsigned `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(1).overflowing_sub_unsigned(uint!(2)), (int!(-1), false));
assert_eq!(int::MAX.overflowing_sub_unsigned(uint::MAX), (int::MIN, false));
assert_eq!((int::MIN + int!(2)).overflowing_sub_unsigned(uint!(3)), (int::MAX, true));
```
