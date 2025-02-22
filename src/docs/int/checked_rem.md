Checked integer remainder. Computes `self % rhs`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(5).checked_rem(int!(2)), Some(int!(1)));
assert_eq!(int!(5).checked_rem(int!(0)), None);
assert_eq!(int::MIN.checked_rem(int!(-1)), None);
```
