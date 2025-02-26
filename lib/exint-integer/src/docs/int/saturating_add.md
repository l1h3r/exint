Saturating integer addition. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).saturating_add(int!(1)), int!(101));
assert_eq!(int::MAX.saturating_add(int!(100)), int::MAX);
assert_eq!(int::MIN.saturating_add(int!(-1)), int::MIN);
```
