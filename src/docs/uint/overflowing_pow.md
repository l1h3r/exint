Raises self to the power of `exp`, using exponentiation by squaring.

Returns a tuple of the exponentiation along with a bool indicating whether an
overflow happened.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(3_u24.overflowing_pow(5), (243_u24, false));
assert_eq!(3_u8.overflowing_pow(6), (217_u8, true));
# }
```
