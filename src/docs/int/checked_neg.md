Checked negation. Computes `-self`,
returning `None` if `self == MIN`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(5).checked_neg(), Some(int!(-5)));
assert_eq!(int::MIN.checked_neg(), None);
```
