Saturating integer multiplication. Computes `self * rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(10).saturating_mul(int!(12)), int!(120));
assert_eq!(int::MAX.saturating_mul(int!(10)), int::MAX);
assert_eq!(int::MIN.saturating_mul(int!(10)), int::MIN);
```
