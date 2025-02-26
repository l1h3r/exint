Computes the absolute value of `self` without any wrapping or panicking.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).unsigned_abs(), uint!(100));
assert_eq!(int!(-100).unsigned_abs(), uint!(100));
assert_eq!(int!(-128).unsigned_abs(), uint!(128));
```
