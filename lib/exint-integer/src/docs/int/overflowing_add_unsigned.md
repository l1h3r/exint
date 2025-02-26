Calculates `self` + `rhs` with an unsigned `rhs`.

Returns a tuple of the addition along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(1).overflowing_add_unsigned(uint!(2)), (int!(3), false));
assert_eq!(int::MIN.overflowing_add_unsigned(uint::MAX), (int::MAX, false));
assert_eq!((int::MAX - int!(2)).overflowing_add_unsigned(uint!(3)), (int::MIN, true));
```
