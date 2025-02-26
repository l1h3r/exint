Calculates `self` - `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(5).overflowing_sub(int!(2)), (int!(3), false));
assert_eq!(int::MIN.overflowing_sub(int!(1)), (int::MAX, true));
```
