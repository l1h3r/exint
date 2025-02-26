Calculates the middle point of `self` and `rhs`.

`midpoint(a, b)` is `(a + b) >> 1` as if it were performed in a
sufficiently-large signed integral type. This implies that the result is always
rounded towards negative infinity and that no overflow will ever occur.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(0).midpoint(int!(4)), int!(2));
assert_eq!(int!(-1).midpoint(int!(2)), int!(0));
assert_eq!(int!(-7).midpoint(int!(0)), int!(-3));
assert_eq!(int!(0).midpoint(int!(-7)), int!(-3));
assert_eq!(int!(0).midpoint(int!(7)), int!(3));
```
