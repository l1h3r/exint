Raises self to the power of `exp`, using exponentiation by squaring.

Returns a tuple of the exponentiation along with a bool indicating whether an
overflow happened.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(3_i24.overflowing_pow(4), (81_i24, false));
assert_eq!(3_i8.overflowing_pow(5), (-13_i8, true));
# }
```
