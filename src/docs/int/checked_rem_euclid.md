Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(5).checked_rem_euclid(int!(2)), Some(int!(1)));
assert_eq!(int!(5).checked_rem_euclid(int!(0)), None);
assert_eq!(int::MIN.checked_rem_euclid(int!(-1)), None);
```
