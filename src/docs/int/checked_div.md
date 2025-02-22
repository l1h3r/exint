Checked integer division. Computes `self / rhs`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!((int::MIN + int!(1)).checked_div(int!(-1)), Some(int::MAX));
assert_eq!(int::MIN.checked_div(int!(-1)), None);
assert_eq!(int!(1).checked_div(int!(0)), None);
```
