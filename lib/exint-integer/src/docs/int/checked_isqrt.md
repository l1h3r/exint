Returns the square root of the number, rounded down.

Returns `None` if `self` is negative.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(10).checked_isqrt(), Some(int!(3)));
```
