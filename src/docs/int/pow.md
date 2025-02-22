Raises self to the power of `exp`, using exponentiation by squaring.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(2).pow(5), int!(32));
```
