Checked integer addition. Computes `self + rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!((int::MAX - int!(2)).checked_add(int!(1)), Some(int::MAX - int!(1)));
assert_eq!((int::MAX - int!(2)).checked_add(int!(3)), None);
```
