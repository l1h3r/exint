Checked absolute value. Computes `self.abs()`,
returning `None` if `self == MIN`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(-5).checked_abs(), Some(int!(5)));
assert_eq!(int::MIN.checked_abs(), None);
```
