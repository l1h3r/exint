Returns `true` if `self` is an integer multiple of `rhs`, and false otherwise.

This function is equivalent to `self % rhs == 0`, except that it will not panic
for `rhs == 0`. Instead, `0.is_multiple_of(0) == true`, and for any non-zero
`n`, `n.is_multiple_of(0) == false`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert!(uint!(6).is_multiple_of(uint!(2)));
assert!(!uint!(5).is_multiple_of(uint!(2)));

assert!(uint!(0).is_multiple_of(uint!(0)));
assert!(!uint!(6).is_multiple_of(uint!(0)));
```
