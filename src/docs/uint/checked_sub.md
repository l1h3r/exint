Checked integer subtraction. Computes `self - rhs`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(1).checked_sub(uint!(1)), Some(uint!(0)));
assert_eq!(uint!(0).checked_sub(uint!(1)), None);
```
