Saturating integer subtraction. Computes `self - rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(100).saturating_sub(uint!(27)), uint!(73));
assert_eq!(uint!(13).saturating_sub(uint!(127)), uint!(0));
```
