Saturating integer addition. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(100).saturating_add(uint!(1)), uint!(101));
assert_eq!(uint::MAX.saturating_add(uint!(127)), uint::MAX);
```
