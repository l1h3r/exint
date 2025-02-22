Calculates `self` - `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(5).overflowing_sub(uint!(2)), (uint!(3), false));
assert_eq!(uint!(0).overflowing_sub(uint!(1)), (uint::MAX, true));
```
