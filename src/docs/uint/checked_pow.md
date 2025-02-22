Checked exponentiation. Computes `self.pow(exp)`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(2).checked_pow(5), Some(uint!(32)));
assert_eq!(uint::MAX.checked_pow(2), None);
```
