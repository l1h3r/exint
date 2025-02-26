Checked negation. Computes `-self`,
returning `None` unless `self == 0`.

Note that negating any positive integer will overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(0).checked_neg(), Some(uint!(0)));
assert_eq!(uint!(1).checked_neg(), None);
```
