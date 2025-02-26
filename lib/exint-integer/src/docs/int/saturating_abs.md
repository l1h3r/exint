Saturating absolute value. Computes `self.abs()`,
returning `MAX` if `self == MIN` instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).saturating_abs(), int!(100));
assert_eq!(int!(-100).saturating_abs(), int!(100));
assert_eq!(int::MIN.saturating_abs(), int::MAX);
assert_eq!((int::MIN + int!(1)).saturating_abs(), int::MAX);
```
