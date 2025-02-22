Negates self in an overflowing fashion.

Returns `!self + 1` using wrapping operations to return the value that
represents the negation of this unsigned value. Note that for positive unsigned
values overflow always occurs, but negating 0 does not overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(0).overflowing_neg(), (uint!(0), false));
assert_eq!(uint!(2).overflowing_neg(), (uint::MAX - uint!(1), true));
```
