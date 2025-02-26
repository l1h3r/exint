Returns the base 2 logarithm of the number, rounded down.

Returns `None` if the number is negative or zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(2).checked_ilog2(), Some(1));
```
