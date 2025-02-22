Returns `true` if and only if `self == 2^k` for some `k`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert!(uint!(16).is_power_of_two());
assert!(!uint!(10).is_power_of_two());
```
