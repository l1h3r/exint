Saturating integer exponentiation. Computes `self.pow(exp)`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(4).saturating_pow(3), uint!(64));
assert_eq!(uint::MAX.saturating_pow(2), uint::MAX);
```
