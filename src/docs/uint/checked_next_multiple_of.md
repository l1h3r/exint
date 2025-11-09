Calculates the smallest value greater than or equal to `self` that is a multiple
of `rhs`. Returns `None` if `rhs` is zero or the operation would result in
overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(16_u24.checked_next_multiple_of(8_u24), Some(16_u24));
assert_eq!(23_u24.checked_next_multiple_of(8_u24), Some(24_u24));
assert_eq!(1_u24.checked_next_multiple_of(0_u24), None);
assert_eq!(u24::MAX.checked_next_multiple_of(2_u24), None);
# }
```
