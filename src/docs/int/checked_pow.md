Checked exponentiation. Computes `self.pow(exp)`,
returning `None` if overflow occurred.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(8).checked_pow(2), Some(int!(64)));
assert_eq!(int::MAX.checked_pow(2), None);
```
