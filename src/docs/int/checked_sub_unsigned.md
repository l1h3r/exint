Checked subtraction with an unsigned integer. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(1).checked_sub_unsigned(uint!(2)), Some(int!(-1)));
assert_eq!((int::MIN + int!(2)).checked_sub_unsigned(uint!(3)), None);
```
