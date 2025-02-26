Saturating integer subtraction. Computes `self - rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).saturating_sub(int!(127)), int!(-27));
assert_eq!(int::MIN.saturating_sub(int!(100)), int::MIN);
assert_eq!(int::MAX.saturating_sub(int!(-1)), int::MAX);
```
