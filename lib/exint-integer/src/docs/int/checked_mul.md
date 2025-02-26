Checked integer multiplication. Computes `self * rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int::MAX.checked_mul(int!(1)), Some(int::MAX));
assert_eq!(int::MAX.checked_mul(int!(2)), None);
```
