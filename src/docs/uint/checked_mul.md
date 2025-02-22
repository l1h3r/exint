Checked integer multiplication. Computes `self * rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(5).checked_mul(uint!(1)), Some(uint!(5)));
assert_eq!(uint::MAX.checked_mul(uint!(2)), None);
```
