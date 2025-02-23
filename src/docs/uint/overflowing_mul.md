Calculates the multiplication of `self` and `rhs`.

Returns a tuple of the multiplication along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).overflowing_mul(uint!(2)), (uint!(10), false));
assert_eq!(uint!(1000000000).overflowing_mul(uint!(10)), (uint!(1410065408), true));
assert_eq!(uint::MAX.overflowing_mul(uint!(2)), (uint::MAX - uint!(1), true));
```
