Calculates `self` + `rhs`.

Returns a tuple of the addition along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(5).overflowing_add(int!(2)), (int!(7), false));
assert_eq!(int::MAX.overflowing_add(int!(1)), (int::MIN, true));
```
