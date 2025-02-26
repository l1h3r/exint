Checked integer subtraction. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!((int::MIN + int!(2)).checked_sub(int!(1)), Some(int::MIN + int!(1)));
assert_eq!((int::MIN + int!(2)).checked_sub(int!(3)), None);
```
