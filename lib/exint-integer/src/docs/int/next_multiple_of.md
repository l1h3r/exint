If `rhs` is positive, calculates the smallest value greater than or equal to
`self` that is a multiple of `rhs`. If `rhs` is negative, calculates the largest
value less than or equal to `self` that is a multiple of `rhs`.

# Panics

This function will panic if `rhs` is zero.

## Overflow behavior

On overflow, this function will panic if overflow checks are enabled (default in
debug mode) and wrap if overflow checks are disabled (default in release mode).

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(16).next_multiple_of(int!(8)), int!(16));
assert_eq!(int!(23).next_multiple_of(int!(8)), int!(24));
assert_eq!(int!(16).next_multiple_of(int!(-8)), int!(16));
assert_eq!(int!(23).next_multiple_of(int!(-8)), int!(16));
assert_eq!(int!(-16).next_multiple_of(int!(8)), int!(-16));
assert_eq!(int!(-23).next_multiple_of(int!(8)), int!(-16));
assert_eq!(int!(-16).next_multiple_of(int!(-8)), int!(-16));
assert_eq!(int!(-23).next_multiple_of(int!(-8)), int!(-24));
```
