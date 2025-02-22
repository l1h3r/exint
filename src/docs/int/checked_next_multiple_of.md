If `rhs` is positive, calculates the smallest value greater than or equal to
`self` that is a multiple of `rhs`. If `rhs` is negative, calculates the largest
value less than or equal to `self` that is a multiple of `rhs`. Returns `None`
if `rhs` is zero or the operation would result in overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(16).checked_next_multiple_of(int!(8)), Some(int!(16)));
assert_eq!(int!(23).checked_next_multiple_of(int!(8)), Some(int!(24)));
assert_eq!(int!(16).checked_next_multiple_of(int!(-8)), Some(int!(16)));
assert_eq!(int!(23).checked_next_multiple_of(int!(-8)), Some(int!(16)));
assert_eq!(int!(-16).checked_next_multiple_of(int!(8)), Some(int!(-16)));
assert_eq!(int!(-23).checked_next_multiple_of(int!(8)), Some(int!(-16)));
assert_eq!(int!(-16).checked_next_multiple_of(int!(-8)), Some(int!(-16)));
assert_eq!(int!(-23).checked_next_multiple_of(int!(-8)), Some(int!(-24)));
assert_eq!(int!(1).checked_next_multiple_of(int!(0)), None);
assert_eq!(int::MAX.checked_next_multiple_of(int!(2)), None);
```
