Wrapping (modular) addition. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).wrapping_add(int!(27)), int!(127));
assert_eq!(int::MAX.wrapping_add(int!(2)), int::MIN + int!(1));
```
