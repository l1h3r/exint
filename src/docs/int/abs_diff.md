Computes the absolute difference between `self` and `rhs`.

This function always returns the correct answer without overflow or panics by
returning an unsigned integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(100).abs_diff(int!(80)), uint!(20));
assert_eq!(int!(100).abs_diff(int!(110)), uint!(10));
assert_eq!(int!(-100).abs_diff(int!(80)), uint!(180));
assert_eq!(int!(-100).abs_diff(int!(-120)), uint!(20));
assert_eq!(int::MIN.abs_diff(int::MAX), uint::MAX);
```
