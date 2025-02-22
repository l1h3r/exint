Saturating subtraction with an unsigned integer. Computes `self - rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(100).saturating_sub_unsigned(uint!(127)), int!(-27));
assert_eq!(int::MIN.saturating_sub_unsigned(uint!(100)), int::MIN);
```
