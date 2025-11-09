If `rhs` is positive, calculates the smallest value greater than or equal to
`self` that is a multiple of `rhs`. If `rhs` is negative, calculates the largest
value less than or equal to `self` that is a multiple of `rhs`. Returns `None`
if `rhs` is zero or the operation would result in overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(16_i24.checked_next_multiple_of(8_i24), Some(16_i24));
assert_eq!(23_i24.checked_next_multiple_of(8_i24), Some(24_i24));
assert_eq!(16_i24.checked_next_multiple_of(-8_i24), Some(16_i24));
assert_eq!(23_i24.checked_next_multiple_of(-8_i24), Some(16_i24));
assert_eq!(-16_i24.checked_next_multiple_of(8_i24), Some(-16_i24));
assert_eq!(-23_i24.checked_next_multiple_of(8_i24), Some(-16_i24));
assert_eq!(-16_i24.checked_next_multiple_of(-8_i24), Some(-16_i24));
assert_eq!(-23_i24.checked_next_multiple_of(-8_i24), Some(-24_i24));
assert_eq!(1_i24.checked_next_multiple_of(0_i24), None);
assert_eq!(i24::MAX.checked_next_multiple_of(2_i24), None);
# }
```
