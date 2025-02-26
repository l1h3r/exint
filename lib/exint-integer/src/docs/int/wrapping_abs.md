Wrapping (modular) absolute value. Computes `self.abs()`,
wrapping around at the boundary of the type.

The only case where such wrapping can occur is when one takes the absolute value
of the negative minimal value for the type; this is a positive value that is too
large to represent in the type. In such a case, this function returns `MIN` itself.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).wrapping_abs(), int!(100));
assert_eq!(int!(-100).wrapping_abs(), int!(100));
assert_eq!(int::MIN.wrapping_abs(), int::MIN);
assert_eq!(int!(-128 i8).wrapping_abs().cast_unsigned(), uint!(128 u8));
```
