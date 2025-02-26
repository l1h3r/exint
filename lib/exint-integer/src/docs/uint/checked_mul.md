Checked integer multiplication. Computes `self * rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).checked_mul(uint!(1)), Some(uint!(5)));
assert_eq!(uint::MAX.checked_mul(uint!(2)), None);
```
