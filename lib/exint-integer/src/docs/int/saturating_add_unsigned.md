Saturating addition with an unsigned integer. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(1).saturating_add_unsigned(uint!(2)), int!(3));
assert_eq!(int::MAX.saturating_add_unsigned(uint!(100)), int::MAX);
```
