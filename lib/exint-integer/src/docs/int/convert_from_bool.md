Converts a [`bool`] to [`int`] losslessly.
The resulting value is `0` for `false` and `1` for `true` values.

[`int`]: crate::types::int

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int::from(true), int!(1));
assert_eq!(int::from(false), int!(0));
```
