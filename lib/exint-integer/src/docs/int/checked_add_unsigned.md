Checked addition with an unsigned integer. Computes `self + rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(1).checked_add_unsigned(uint!(2)), Some(int!(3)));
assert_eq!((int::MAX - int!(2)).checked_add_unsigned(uint!(3)), None);
```
