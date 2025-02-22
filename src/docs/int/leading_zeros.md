Returns the number of leading zeros in the binary representation of `self`.

Depending on what you're doing with the value, you might also be interested in the
[`ilog2`] function which returns a consistent number, even if the type widens.

[`ilog2`]: Self::ilog2

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(-1).leading_zeros(), 0);
```
