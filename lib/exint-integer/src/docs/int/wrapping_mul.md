Wrapping (modular) multiplication. Computes `self * rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(10).wrapping_mul(int!(12)), int!(120));
assert_eq!(int!(11 i8).wrapping_mul(int!(12 i8)), int!(-124 i8));
```
