Raises self to the power of `exp`, using exponentiation by squaring.

Returns a tuple of the exponentiation along with a bool indicating whether an
overflow happened.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(3).overflowing_pow(5), (uint!(243), false));
assert_eq!(uint!(3 u8).overflowing_pow(6), (uint!(217 u8), true));
```
