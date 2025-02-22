Returns `true` if `self` is positive and `false` if the number is zero or negative.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert!(int!(10).is_positive());
assert!(!int!(-10).is_positive());
```
