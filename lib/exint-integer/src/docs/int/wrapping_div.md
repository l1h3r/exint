Wrapping (modular) division. Computes `self / rhs`,
wrapping around at the boundary of the type.

The only case where such wrapping can occur is when one divides `MIN / -1` on a
signed type (where `MIN` is the negative minimal value for the type); this is
equivalent to `-MIN`, a positive value that is too large to represent in the
type. In such a case, this function returns `MIN` itself.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).wrapping_div(int!(10)), int!(10));
assert_eq!(int!(-128 i8).wrapping_div(int!(-1 i8)), int!(-128 i8));
```
