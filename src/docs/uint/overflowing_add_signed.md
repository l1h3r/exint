Calculates `self` + `rhs` with a signed `rhs`.

Returns a tuple of the addition along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(1).overflowing_add_signed(int!(2)), (uint!(3), false));
assert_eq!(uint!(1).overflowing_add_signed(int!(-2)), (uint::MAX, true));
assert_eq!((uint::MAX - uint!(2)).overflowing_add_signed(int!(4)), (uint!(1), true));
```
