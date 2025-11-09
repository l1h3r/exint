Wrapping (modular) absolute value. Computes `self.abs()`,
wrapping around at the boundary of the type.

The only case where such wrapping can occur is when one takes the absolute value
of the negative minimal value for the type; this is a positive value that is too
large to represent in the type. In such a case, this function returns `MIN` itself.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_abs(), 100_i24);
assert_eq!((-100_i24).wrapping_abs(), 100_i24);
assert_eq!(i24::MIN.wrapping_abs(), i24::MIN);
assert_eq!((-128_i8).wrapping_abs().cast_unsigned(), 128_u8);
# }
```
