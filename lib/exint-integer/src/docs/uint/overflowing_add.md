Calculates `self` + `rhs`.

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
assert_eq!(uint!(5).overflowing_add(uint!(2)), (uint!(7), false));
assert_eq!(uint::MAX.overflowing_add(uint!(1)), (uint!(0), true));
```
