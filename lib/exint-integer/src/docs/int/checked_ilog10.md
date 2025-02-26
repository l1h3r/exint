Returns the base 10 logarithm of the number, rounded down.

Returns `None` if the number is negative or zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(10).checked_ilog10(), Some(1));
```
