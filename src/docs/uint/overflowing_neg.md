Negates self in an overflowing fashion.

Returns `!self + 1` using wrapping operations to return the value that
represents the negation of this unsigned value. Note that for positive unsigned
values overflow always occurs, but negating 0 does not overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.overflowing_neg(), (0_u24, false));
assert_eq!(2_u24.overflowing_neg(), (u24::MAX - 1_u24, true));
# }
```
