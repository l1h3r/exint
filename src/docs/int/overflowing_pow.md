Raises self to the power of `exp`, using exponentiation by squaring.

Returns a tuple of the exponentiation along with a bool indicating whether an
overflow happened.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(3).overflowing_pow(4), (int!(81), false));
assert_eq!(3i8.overflowing_pow(5), (-13, true));
```
