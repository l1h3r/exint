Converts a [`bool`] to [`uint`] losslessly.
The resulting value is `0` for `false` and `1` for `true` values.

[`uint`]: crate::types::uint

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint::from(true), uint!(1));
assert_eq!(uint::from(false), uint!(0));
```
