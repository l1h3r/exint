Returns `true` if `self` is negative and `false` if the number is zero or positive.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert!(int!(-10).is_negative());
assert!(!int!(10).is_negative());
```
