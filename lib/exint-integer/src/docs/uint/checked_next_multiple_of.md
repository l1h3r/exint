Calculates the smallest value greater than or equal to `self` that is a multiple
of `rhs`. Returns `None` if `rhs` is zero or the operation would result in
overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(16).checked_next_multiple_of(uint!(8)), Some(uint!(16)));
assert_eq!(uint!(23).checked_next_multiple_of(uint!(8)), Some(uint!(24)));
assert_eq!(uint!(1).checked_next_multiple_of(uint!(0)), None);
assert_eq!(uint::MAX.checked_next_multiple_of(uint!(2)), None);
```
