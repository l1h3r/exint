Checked addition with a signed integer. Computes `self + rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(1).checked_add_signed(int!(2)), Some(uint!(3)));
assert_eq!(uint!(1).checked_add_signed(int!(-2)), None);
assert_eq!((uint::MAX - uint!(2)).checked_add_signed(int!(3)), None);
```
